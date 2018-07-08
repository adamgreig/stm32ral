#!/usr/bin/env python3
"""
stm32ral.py
Copyright 2018 Adam Greig
Licensed under MIT and Apache 2.0, see LICENSE_MIT and LICENSE_APACHE.
"""

import os
import argparse
import itertools
import subprocess
import multiprocessing
import xml.etree.ElementTree as ET
from fnmatch import fnmatch


CRATE_LIB_PREAMBLE = """\
// Copyright 2018 Adam Greig
// See LICENSE-APACHE and LICENSE-MIT for license details.

//! This project provides a register access layer (RAL) for all
//! STM32 microcontrollers.
//!
//! When built, you must specify a device feature, such as `stm32f405`.
//! This will cause all modules in that device's module to be re-exported
//! from the top level, so that for example `stm32ral::gpio` will resolve to
//! `stm32ral::stm32f4::stm32f405::gpio`.
//!
//! In the generated documentation, all devices are visible inside their family
//! modules, but when built for a specific device, only that devices' constants
//! will be available.
//!
//! See the [README](https://github.com/adamgreig/stm32ral) for example usage.

#![no_std]

#[macro_use]
mod register;

/// Set the interrupt handler for a specific interrupt.
///
/// Call with `interrupt!(NAME, my_handler);`, where `NAME` must be in
/// `stm32ral::interrupts::Interrupt`, and `my_handler` must have type `fn()`.
///
/// This macro is only available with the `rt` feature.
#[cfg(any(feature="doc", feature="rt"))]
#[macro_export]
macro_rules! interrupt {
    ($name:ident, $handler:path) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name() {
            let _ = $crate::interrupts::Interrupt::$name;
            let f: fn() = $handler;
            f()
        }
    };
}


pub use register::{RORegister, WORegister, RWRegister};
pub use register::{UnsafeRORegister, UnsafeRWRegister, UnsafeWORegister};

"""


CRATE_CARGO_TOML_PREAMBLE = """\
[package]
name = "stm32ral"
version = "0.1.0"
authors = ["Adam Greig <adam@adamgreig.com>"]
description = "Register access layer for all STM32 microcontrollers"
repository = "https://github.com/adamgreig/stm32ral"
documentation = "https://docs.rs/stm32ral"
readme = "README.md"
keywords = ["stm32", "embedded", "no_std"]
categories = ["embedded", "no-std"]
license = "MIT/Apache-2.0"

[package.metadata.docs.rs]
features = ["doc"]

[dependencies]
bare-metal = "0.2.0"

[dependencies.cortex-m-rt]
optional = true
version = "0.5.1"

[features]
default = []
unsafe = []
rt = ["cortex-m-rt/device"]
doc = []
"""


BUILD_RS_TEMPLATE = """\
use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {{
    if env::var_os("CARGO_FEATURE_RT").is_some() {{
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={{}}", out.display());
        let device_file = {device_clauses};
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={{}}", device_file);
    }}
    println!("cargo:rerun-if-changed=build.rs");
}}
"""


UNSAFE_REGISTERS = [
    # DMA peripheral and memory address registers
    "S?PAR", "S?M?AR", "CPAR?", "CMAR?",

    # DMA2D address registers
    "FGMAR", "BGMAR", "FGCMAR", "BGCMAR", "OMAR",

    # LTDC frame buffer address register
    "L?CFBAR",

    # USB OTG DMA address register
    "DIEPDMA*", "DOEPDMA*", "HCDMA*",

    # Ethernet DMA descriptor list address register
    "DMARDLAR", "DMATDLAR",

    # Cache operations
    "ICIALLU", "?C?MVA?", "DC?SW", "DCCIMVAC", "DCCISW", "BPIALL",
]


class Node:
    """
    A node in the overall graph.
    """
    pass


class EnumeratedValue(Node):
    """
    Represents a possible named value for a field.

    Has a name, description, and value.
    Belongs to one or more parent Fields.
    """
    def __init__(self, name, desc, value):
        self.name = name
        self.desc = desc
        self.value = value

    def to_dict(self):
        return {"name": self.name, "desc": self.desc, "value": self.value}

    def to_rust(self, field_width):
        return f"""
        /// {self.desc}
        pub const {self.name}: u32 = 0b{self.value:0{field_width}b};"""

    @classmethod
    def from_svd(cls, svd, node):
        name = get_string(node, 'name')
        desc = get_string(node, 'description')
        value = get_int(node, 'value')
        return cls(name, desc, value)

    def __eq__(self, other):
        return (
            self.name == other.name and
            self.value == other.value and
            self.desc == other.desc)

    def __lt__(self, other):
        return self.value < other.value


class EnumeratedValues(Node):
    """
    Represents possible named values for a field, emitted as a concrete Enum.

    Contains many child EnumeratedValues.
    """
    def __init__(self, name):
        self.name = name
        self.values = []

    def to_dict(self):
        return {"name": self.name,
                "values": [v.to_dict() for v in self.values]}

    def to_rust(self, field_width):
        values = "\n".join(v.to_rust(field_width) for v in self.values)
        return f"""\
        pub mod {self.name} {{
            {values}
        }}"""

    @classmethod
    def from_svd(cls, svd, node):
        usage = get_string(node, 'usage')
        if usage == "read":
            name = "R"
        elif usage == "write":
            name = "W"
        else:
            name = "RW"
        evs = cls(name)
        for ev in node.findall('enumeratedValue'):
            evs.values.append(EnumeratedValue.from_svd(svd, ev))
        return evs

    @classmethod
    def empty(cls, name):
        return cls(name)

    def __eq__(self, other):
        return (
            self.name == other.name and
            len(self.values) == len(other.values) and
            all(v1 == v2 for v1, v2
                in zip(sorted(self.values), sorted(other.values))))


class EnumeratedValuesLink(Node):
    """
    Represents an EnumeratedValues enum which is included with 'use'.
    """
    def __init__(self, field, evs):
        self.field = field
        self.evs = evs

    def to_dict(self):
        return {"field": self.field.name, "evs": self.evs.name}

    def to_rust(self, field_width):
        return f"pub use ::super::{self.field.name}::{self.evs.name};"

    def __eq__(self, other):
        return self.evs.__eq__(other)

    @property
    def name(self):
        return self.evs.name

    @property
    def values(self):
        return self.evs.values


class Field(Node):
    """
    Represents a field in a register.

    Has a name, description, width, offset, access, and three child
    EnumeratedValues: R, W, and RW.
    Belongs to a parent Register.
    May contain one or more child EnumeratedValues.
    """
    def __init__(self, name, desc, width, offset, access, r, w, rw):
        self.name = name
        self.desc = desc
        self.width = width
        self.offset = offset
        self.access = access
        self.r = r
        self.w = w
        self.rw = rw

    def to_dict(self):
        return {"name": self.name, "desc": self.desc, "width": self.width,
                "offset": self.offset, "access": self.access,
                "r": self.r.to_dict(), "w": self.w.to_dict(),
                "rw": self.rw.to_dict()}

    def to_rust(self):
        mask = 2**self.width - 1
        return f"""
        /// {self.desc}
        pub mod {self.name} {{
            pub const offset: u32 = {self.offset};
            pub const mask: u32 = 0b{mask:b} << offset;
            {self.r.to_rust(self.width)}
            {self.w.to_rust(self.width)}
            {self.rw.to_rust(self.width)}
        }}"""

    @classmethod
    def from_svd(cls, svd, node, ctx):
        ctx = ctx.inherit(node)
        name = get_string(node, 'name')
        desc = get_string(node, 'description')
        width = get_int(node, 'bitWidth')
        offset = get_int(node, 'bitOffset')
        access = ctx.access
        r = EnumeratedValues.empty("R")
        w = EnumeratedValues.empty("W")
        rw = EnumeratedValues.empty("RW")
        for evs in node.findall('enumeratedValues'):
            if 'derivedFrom' in evs.attrib:
                df = evs.attrib['derivedFrom']
                evs = svd.find(f".//enumeratedValues[name='{df}']")
                if evs is None:
                    raise ValueError(f"Can't find derivedFrom {df}")
            evs = EnumeratedValues.from_svd(svd, evs)
            evsname = evs.name
            if evsname == "R":
                r = evs
            elif evsname == "W":
                w = evs
            else:
                rw = evs
        field = cls(name, desc, width, offset, access, r, w, rw)
        return field

    def __eq__(self, other):
        return (
            self.name == other.name and
            self.width == other.width and
            self.offset == other.offset and
            self.access == other.access and
            self.r == other.r and self.w == other.w and self.rw == other.rw)

    def __lt__(self, other):
        return (self.offset, self.name) < (other.offset, other.name)


class FieldLink(Node):
    """
    A Field which outputs a `use` statement instead of a module.
    """
    def __init__(self, parent, path):
        self.parent = parent
        self.path = path
        self.r = parent.r
        self.w = parent.w
        self.rw = parent.rw

    def to_dict(self):
        return {"parent": self.parent.name, "path": self.path}

    def to_rust(self):
        return f"pub use {self.path}::{self.parent.name};"

    def __lt__(self, other):
        return self.parent.__lt__(other)

    def __eq__(self, other):
        return self.parent.__eq__(other)

    @property
    def name(self):
        return self.parent.name

    @property
    def desc(self):
        return self.parent.desc

    @property
    def width(self):
        return self.parent.width

    @property
    def offset(self):
        return self.parent.offset

    @property
    def access(self):
        return self.parent.access


class RegisterCtx:
    """
    The inheritance context for register properties.
    Equivalent to an SVD `registerPropertiesGroup`.
    """
    def __init__(self, size, access, reset_value, reset_mask):
        self.size = size
        self.access = access
        self.reset_value = reset_value
        self.reset_mask = reset_mask

    @classmethod
    def empty(cls):
        """Make an empty context."""
        return cls(None, None, None, None)

    def copy(self):
        """Return a copy of self."""
        return RegisterCtx(self.size, self.access, self.reset_value,
                           self.reset_mask)

    def update_from_node(self, node):
        """
        Copies any specified properties from the given node into self,
        leaving unspecified properties unchanged. Returns self for
        easier chaining.
        """
        size = get_int(node, 'size')
        access = get_string(node, 'access')
        reset_value = get_int(node, 'resetValue')
        reset_mask = get_int(node, 'resetMask')
        if size is not None:
            self.size = size
        if access is not None:
            self.access = access
        if reset_value is not None:
            self.reset_value = reset_value
        if reset_mask is not None:
            self.reset_mask = reset_mask
        return self

    def inherit(self, node):
        """Return a copy of self which has been updated using `node`."""
        return self.copy().update_from_node(node)


class Register(Node):
    """
    Represents a register in a peripheral.

    Has a name, description, offset, size, access, reset value and mask.
    Belongs to a parent Peripheral.
    May contain one or more child Fields.
    """
    def __init__(self, name, desc, offset, size, access, reset_value,
                 reset_mask):
        self.name = name
        self.desc = desc
        self.offset = offset
        self.size = size
        self.access = access
        self.reset_value = reset_value
        self.reset_mask = reset_mask
        self.fields = []

    def to_dict(self):
        return {"name": self.name, "desc": self.desc, "offset": self.offset,
                "size": self.size, "access": self.access,
                "reset_value": self.reset_value, "reset_mask": self.reset_mask,
                "fields": [x.to_dict() for x in self.fields]}

    def to_rust_mod(self):
        """
        Returns a Rust public module containing a public module for each
        of this register's fields.
        """
        fields = "\n".join(f.to_rust() for f in self.fields)
        return f"""
        /// {self.desc}
        pub mod {self.name} {{
            {fields}
        }}"""

    def to_regtype(self):
        """
        Return the type of register (RORegister, UnsafeWORegister, etc)
        used for this Register.
        """
        regtype = {"read-only": "RORegister", "write-only": "WORegister",
                   "read-write": "RWRegister"}[self.access]
        for unsafe in UNSAFE_REGISTERS:
            if fnmatch(self.name, unsafe):
                regtype = "Unsafe" + regtype
                break
        return regtype

    def to_rust_struct_entry(self):
        """Returns the RegisterBlock entry for this register."""
        regtype = self.to_regtype()
        return f"""
        /// {self.desc}
        pub {self.name}: {regtype}<u{self.size}>,
        """

    @classmethod
    def from_svd(cls, svd, node, ctx):
        ctx = ctx.inherit(node)
        name = get_string(node, 'name')
        desc = get_string(node, 'description')
        offset = get_int(node, 'addressOffset')
        register = cls(name, desc, offset, ctx.size, ctx.access,
                       ctx.reset_value, ctx.reset_mask)
        fields = node.find('fields')
        if fields is not None:
            for field in fields.findall('field'):
                register.fields.append(Field.from_svd(svd, field, ctx))
        if register.access is None:
            # This happens if access is defined per-field, typically because
            # there is one or two read-write among many read-only registers.
            field_accesses = [f.access for f in register.fields]
            if all(access == "read-only" for access in field_accesses):
                register.access = "read-only"
            elif all(access == "write-only" for access in field_accesses):
                register.access = "write-only"
            else:
                register.access = "read-write"
        return register

    def __eq__(self, other):
        return (
            self.name == other.name and
            self.offset == other.offset and
            self.size == other.size and
            self.access == other.access and
            sorted(self.fields) == sorted(other.fields)
        )

    def __lt__(self, other):
        return (self.offset, self.name) < (other.offset, other.name)

    def refactor_common_field_values(self):
        """
        Go through all fields in this register and where two fields have the
        same set of enumated values, replace the latter's with a link to the
        former's.
        """
        replace = []
        to_replace = set()
        fields = enumerate(self.fields)
        for (idx1, f1), (idx2, f2) in itertools.combinations(fields, 2):
            if f1 is f2 or idx1 in to_replace or idx2 in to_replace:
                continue
            if f1.r == f2.r and f1.r.values:
                replace.append((idx1, idx2, 'r'))
                to_replace.add(idx2)
            if f1.w == f2.w and f1.w.values:
                replace.append((idx1, idx2, 'w'))
                to_replace.add(idx2)
            if f1.rw == f2.rw and f1.rw.values:
                replace.append((idx1, idx2, 'rw'))
                to_replace.add(idx2)
        for idx1, idx2, name in replace:
            f1 = self.fields[idx1]
            evs1 = getattr(f1, name)
            f2 = EnumeratedValuesLink(f1, evs1)
            setattr(self.fields[idx2], name, f2)

    def consume(self, other, parent):
        """
        Adds any fields from other to self, and adjusts self's name to the
        common prefix of the two names, if such a prefix is at least
        2 letters long.
        """
        my_field_names = set(f.name for f in self.fields)
        for field in other.fields:
            if field.name not in my_field_names:
                self.fields.append(field)
        self.desc = "\n/// ".join([
            f"{self.name} and {other.name}",
            f"{self.name}: {self.desc}",
            f"{other.name}: {other.desc}",
        ])
        self.size = max(self.size, other.size)
        newname = os.path.commonprefix((self.name, other.name)).strip("_")
        if newname != self.name and len(newname) >= 2:
            prefix = "_".join(self.name.split("_")[:-1])
            if newname != prefix:
                print(f"Warning [{parent.name}]: suspected failure of name "
                      f"compaction: {self.name}+{other.name}->{newname}")
            if newname not in [r.name for r in parent.registers]:
                self.name = newname
            else:
                print(f"Warning [{parent.name}]: {self.name}+{other.name} "
                      "aliasing produces name conflict, not renaming")
        self.access = "read-write"


class PeripheralInstance(Node):
    """
    Represents a specific peripheral instance in a device.

    For example, GPIOA is a PeripheralInstance, while GPIO is a
    PeripheralPrototype which would contain GPIOA. For many
    peripherals there is a single PeripheralPrototype containing
    a single PeripheralInstance.

    Has a name and base address.
    Belongs to a parent PeripheralPrototype.
    """
    def __init__(self, name, addr, reset_values):
        self.name = name
        self.addr = addr
        self.reset_values = reset_values

    def to_dict(self):
        return {"name": self.name, "addr": self.addr,
                "reset_values": self.reset_values}

    def to_rust(self, registers):
        tname = self.name.title().replace("_", "")
        registers = {r.offset: r.name for r in registers}
        resets = ", ".join(
            f"{registers[k]}: 0x{v:08X}" for k, v in self.reset_values.items())
        return f"""
        pub struct {tname} {{ pub reset: ResetValues }}
        impl ::core::ops::Deref for {tname} {{
            type Target = RegisterBlock;
            fn deref(&self) -> &RegisterBlock {{
                unsafe {{ &*(0x{self.addr:08x} as *const _) }}
            }}
        }}
        pub const {self.name.upper()}: {tname} =
            {tname} {{ reset: ResetValues {{ {resets} }} }};"""

    def __lt__(self, other):
        return self.name < other.name


class PeripheralPrototype(Node):
    """
    Represents a generic peripheral with registers.

    Has a name and description.
    Belongs to a parent Device.
    Contains child PeripheralInstances and Registers.

    Also contains a list of device names which contain this peripheral,
    used to ensure shared peripherals are only compiled when the crate
    is built for a device which uses them.
    """
    def __init__(self, name, desc):
        self.name = name.lower()
        self.desc = desc
        self.registers = []
        self.instances = []
        self.parent_device_names = []

    def to_dict(self):
        return {"name": self.name, "desc": self.desc,
                "registers": [x.to_dict() for x in self.registers],
                "instances": [x.to_dict() for x in self.instances]}

    def to_rust_register_block(self):
        """Creates a RegisterBlock for this peripheral."""
        lines = []
        address = 0
        reservedctr = 1
        for register in sorted(self.registers):
            if register.offset < address:
                raise RuntimeError("Unexpected register aliasing")
            if register.offset != address:
                gaps = []
                u32s = (register.offset - address) // 4
                if u32s != 0:
                    gaps.append(f"[u32; {u32s}]")
                    address += u32s * 4
                u16s = (register.offset - address) // 2
                if u16s != 0:
                    gaps.append(f"[u16; {u16s}]")
                    address += u16s * 2
                u8s = register.offset - address
                if u8s != 0:
                    gaps.append(f"[u8; {u8s}]")
                    address += u8s
                for gaptype in gaps:
                    lines.append(f"_reserved{reservedctr}: {gaptype},")
                    reservedctr += 1
            lines.append(register.to_rust_struct_entry())
            address += register.size // 8
        lines = "\n".join(lines)
        return f"""
        pub struct RegisterBlock {{
            {lines}
        }}"""

    def to_rust_reset_values(self):
        """Creates a ResetValues struct for this peripheral."""
        lines = []
        for register in sorted(self.registers):
            lines.append(f"pub {register.name}: u{register.size},")
        lines = "\n".join(lines)
        return f"""
        pub struct ResetValues {{
            {lines}
        }}"""

    def to_rust_file(self, path):
        """
        Creates {peripheral}.rs in path, and writes all register modules,
        field modules, the register block, and any instances to that file.
        Finally runs rustfmt over the new file.
        """
        regtypes = set(r.to_regtype() for r in self.registers)
        regtypes = ", ".join(regtypes)
        desc = "\n//! ".join(self.desc.split("\n"))
        if len(self.parent_device_names) > 1:
            desc += "\n//!\n"
            desc += "//! Used by: " + ', '.join(
                sorted(set(self.parent_device_names)))
        preamble = "\n".join([
            "#![allow(non_snake_case, non_upper_case_globals)]",
            "#![allow(non_camel_case_types)]",
            f"//! {desc}",
            f"use {{{regtypes}}};",
            "",
        ])
        modules = "\n".join(r.to_rust_mod() for r in self.registers)
        instances = "\n".join(i.to_rust(self.registers)
                              for i in sorted(self.instances))
        fname = os.path.join(path, f"{self.name}.rs")
        with open(fname, "w") as f:
            f.write(preamble)
            f.write(modules)
            f.write(self.to_rust_register_block())
            f.write(self.to_rust_reset_values())
            f.write(instances)
        rustfmt(fname)

    @classmethod
    def from_svd(cls, svd, node, register_ctx):
        name = get_string(node, 'name')
        addr = get_int(node, 'baseAddress')
        desc = get_string(node, 'description')
        registers = node.find('registers')
        if 'derivedFrom' in node.attrib:
            df = node.attrib['derivedFrom']
            df_node = svd.find(f".//peripheral[name='{df}']")
            if df_node is None:
                raise ValueError("Can't find derivedFrom[{df}]")
            desc = get_string(df_node, 'description', default=desc)
            addr = get_int(node, 'baseAddress', addr)
            registers = df_node.find('registers')
            register_ctx = register_ctx.inherit(df_node)
        register_ctx = register_ctx.inherit(node)
        peripheral = cls(name, desc)
        if registers is None:
            raise ValueError(f"No registers found for peripheral {name}")
        ctx = register_ctx
        for register in registers.findall('register'):
            peripheral.registers.append(Register.from_svd(svd, register, ctx))
        resets = {r.offset: r.reset_value for r in peripheral.registers}
        peripheral.instances.append(PeripheralInstance(name, addr, resets))
        return peripheral

    def consume(self, other, parent):
        """
        Adds any PeripheralInstances from other to self, and adjusts self's
        name to the common prefix of the two names, if such a prefix is
        at least 3 letters long.
        """
        self.instances += other.instances
        newname = os.path.commonprefix((self.name, other.name)).strip("_")
        if newname != self.name and len(newname) >= 3:
            if len(newname) < len(self.name) - 3:
                print(f"Warning [{parent.name}]: suspected failure of name "
                      f"compaction: {self.name}+{other.name}->{newname}")
            if newname not in [p.name for p in parent.peripherals]:
                self.name = newname
            else:
                print(f"Warning [{parent.name}]: {self.name}+{other.name}: "
                      f"new name {newname} already exists, not renaming")

    def refactor_common_register_fields(self):
        """
        Go through all registers in this peripheral and where two registers
        have the same set of fields, replace the latter's with links to the
        former's.
        """
        replace = []
        to_replace = set()
        registers = enumerate(self.registers)
        for (idx1, r1), (idx2, r2) in itertools.combinations(registers, 2):
            if r1 is r2 or idx1 in to_replace or idx2 in to_replace:
                continue
            if r1.fields == r2.fields and r1.fields:
                replace.append((idx1, idx2))
                to_replace.add(idx2)
        for idx1, idx2 in replace:
            r1 = self.registers[idx1]
            r2 = self.registers[idx2]
            path = f"super::{r1.name}"
            r2.fields = [FieldLink(f, path) for f in r1.fields]

    def refactor_aliased_registers(self):
        """
        Go through all registers in this peripheral and where two registers
        have the same offset (i.e., are aliased), merge the fields, replace
        the name with the common prefix.
        """
        to_delete = set()
        registers = enumerate(self.registers)
        for (idx1, r1), (idx2, r2) in itertools.combinations(registers, 2):
            if r1 is r2 or idx1 in to_delete or idx2 in to_delete:
                continue
            if r1.offset == r2.offset:
                r1.consume(r2, parent=self)
                to_delete.add(idx2)
        for idx in sorted(to_delete, reverse=True):
            del self.registers[idx]

    def __lt__(self, other):
        return self.name < other.name


class PeripheralPrototypeLink(Node):
    """
    Represents use of an externally defined RegisterBlock and registers,
    with local instances.
    """
    def __init__(self, name, prototype, path):
        """
        `path`: the relative path to the prototype module,
                so that `use {path}::RegisterBlock;` works from the
                context of this module,
                e.g., `super::tim1`, or `super::super::stm32f401::gpio`.
        """
        self.name = name
        self.prototype = prototype
        self.path = path
        self.instances = []

    def to_dict(self):
        return {"prototype": self.prototype.name, "path": self.path,
                "instances": [x.to_dict() for x in self.instances]}

    def to_rust_file(self, path):
        """
        Creates {peripheral}.rs in the path, writes `use` statements
        for all register modules and the register block, and writes any
        instances to that file.
        Finally runs rustfmt over the new file.
        """
        desc = "\n//! ".join(self.prototype.desc.split("\n"))
        preamble = "\n".join([
            "#![allow(non_snake_case, non_upper_case_globals)]",
            "#![allow(non_camel_case_types)]",
            f"//! {desc}",
            "",
            f"pub use {self.path}::RegisterBlock;",
            f"pub use {self.path}::ResetValues;",
            "",
        ])
        registers = "\n".join(f"pub use {self.path}::{m.name};"
                              for m in self.prototype.registers)
        instances = "\n".join(i.to_rust(self.registers)
                              for i in sorted(self.instances))
        fname = os.path.join(path, f"{self.name}.rs")
        with open(fname, "w") as f:
            f.write(preamble)
            f.write(registers)
            f.write("\n")
            f.write(instances)
        rustfmt(fname)

    @classmethod
    def from_peripherals(cls, p1, p2, path):
        plink = cls(p2.name, p1, path)
        plink.instances = p2.instances
        return plink

    @property
    def registers(self):
        return self.prototype.registers

    @property
    def desc(self):
        return self.prototype.desc

    def refactor_common_register_fields(self):
        pass

    def refactor_aliased_registers(self):
        pass

    def __lt__(self, other):
        return self.name < other.name


class CPU(Node):
    """
    Represents the CPU in a device.

    Has a name and nvicPrioBits.
    Belongs to a parent Device.
    """
    def __init__(self, name, nvic_prio_bits):
        self.name = name
        self.nvic_prio_bits = nvic_prio_bits

    def get_architecture(self):
        if self.name == "CM0":
            return "ARMv6-M"
        elif self.name == "CM0+":
            return "ARMv6-M"
        elif self.name == "CM3":
            return "ARMv7-M"
        elif self.name == "CM4":
            return "ARMv7E-M"
        elif self.name == "CM7":
            return "ARMv7E-M"

    def to_dict(self):
        return {"name": self.name, "nvic_prio_bits": self.nvic_prio_bits}

    @classmethod
    def from_svd(cls, svd, node):
        """Load a CPU node from the CPU node of a parsed XML file."""
        name = get_string(node, 'name')
        nvic_prio_bits = node.find('nvicPrioBits').text
        return cls(name, nvic_prio_bits)


class Interrupt(Node):
    """
    Represents an interrupt in a device.

    Has a name, description, and value (interrupt number).
    Belongs to a parent Device.
    """
    def __init__(self, name, desc, value):
        self.name = name
        self.desc = desc
        self.value = value

    def to_dict(self):
        return {"name": self.name, "desc": self.desc, "value": self.value}

    @classmethod
    def from_svd(cls, svd, node):
        name = get_string(node, 'name')
        desc = get_string(node, 'description')
        value = get_int(node, 'value')
        return cls(name, desc, value)

    def __lt__(self, other):
        return self.value < other.value


class Device(Node):
    """
    Represents a device corresponding to a single input SVD file.

    Has a name.
    Contains a child CPU, PeripheralPrototypes, and Interrupts.
    """
    def __init__(self, name, cpu):
        self.name = name.lower().replace("-", "_")
        self.cpu = cpu
        self.peripherals = []
        self.interrupts = []
        self.special = False

    def to_dict(self):
        return {"name": self.name, "cpu": self.cpu.to_dict(),
                "peripherals": [x.to_dict() for x in self.peripherals],
                "interrupts": [x.to_dict() for x in self.interrupts]}

    def to_interrupt_file(self, familypath):
        devicepath = os.path.join(familypath, self.name)
        iname = os.path.join(devicepath, "interrupts.rs")
        with open(iname, "w") as f:
            f.write("extern crate bare_metal;\n")
            f.write('#[cfg(feature="rt")]\nextern "C" {\n')
            for interrupt in self.interrupts:
                f.write(f'    fn {interrupt.name}();\n')
            f.write('}\n\n')
            vectors = []
            offset = 0
            for interrupt in self.interrupts:
                while interrupt.value != offset:
                    vectors.append("Vector { _reserved: 0 },")
                    offset += 1
                vectors.append(f"Vector {{ _handler: {interrupt.name} }},")
                offset += 1
            nvectors = len(vectors)
            vectors = "\n".join(vectors)

            f.write(f"""\
                #[doc(hidden)]
                pub union Vector {{
                    _handler: unsafe extern "C" fn(),
                    _reserved: u32,
                }}

                #[cfg(feature="rt")]
                #[doc(hidden)]
                #[link_section=".vector_table.interrupts"]
                #[no_mangle]
                pub static __INTERRUPTS: [Vector; {nvectors}] = [
                {vectors}
                ];

                /// Available interrupts for this device
                #[repr(u8)]
                #[derive(Clone,Copy)]
                #[allow(non_camel_case_types)]
                pub enum Interrupt {{""")
            for interrupt in self.interrupts:
                f.write(f"/// {interrupt.value}: {interrupt.desc}\n")
                f.write(f"{interrupt.name} = {interrupt.value},\n")
            f.write("}\n")
            f.write("""\
                unsafe impl bare_metal::Nr for Interrupt {
                    #[inline]
                    fn nr(&self) -> u8 {
                        *self as u8
                    }
                }\n""")
        rustfmt(iname)

    def to_files(self, familypath):
        devicepath = os.path.join(familypath, self.name)
        os.makedirs(devicepath, exist_ok=True)
        for peripheral in self.peripherals:
            peripheral.to_rust_file(devicepath)
        pnames = [p.name for p in self.peripherals]
        dupnames = set(name for name in pnames if pnames.count(name) > 1)
        if dupnames:
            print(f"Warning [{self.name}]: duplicate peripherals: ", end='')
            print(dupnames)
        if not self.special:
            self.to_interrupt_file(familypath)
        mname = os.path.join(devicepath, "mod.rs")
        with open(mname, "w") as f:
            f.write(f"//! stm32ral module for {self.name}\n\n")
            prio_bits = self.cpu.nvic_prio_bits
            if not self.special:
                f.write("/// Number of priority bits implemented by the NVIC")
                f.write(f"\npub const NVIC_PRIO_BITS: u8 = {prio_bits};\n\n")
                f.write("/// Interrupt-related magic for this device\n")
                f.write("pub mod interrupts;\n")
                f.write("pub use self::interrupts::Interrupt;\n\n")
            for peripheral in self.peripherals:
                f.write(f"pub mod {peripheral.name};\n")
        rustfmt(mname)
        if not self.special:
            dname = os.path.join(devicepath, "device.x")
            with open(dname, "w") as f:
                for interrupt in self.interrupts:
                    f.write(f"PROVIDE({interrupt.name} = DefaultHandler);\n")

    @classmethod
    def from_svd(cls, svd):
        """Load a Device node and children from a parsed SVD XML file."""
        name = get_string(svd, 'name')
        cpu = CPU.from_svd(svd, svd.find('cpu'))
        device = cls(name, cpu)
        register_ctx = RegisterCtx.empty()
        register_ctx = register_ctx.inherit(svd)
        interrupt_nums = set()
        for interrupt in svd.findall('.//interrupt'):
            interrupt = Interrupt.from_svd(svd, interrupt)
            if interrupt.value in interrupt_nums:
                # Many SVDs have duplicated interrupts. Skip them.
                continue
            device.interrupts.append(interrupt)
            interrupt_nums.add(interrupt.value)
        device.interrupts.sort()
        for peripheral in svd.findall('.//peripheral'):
            device.peripherals.append(
                PeripheralPrototype.from_svd(svd, peripheral, register_ctx))
        for peripheral in device.peripherals:
            peripheral.parent_device_names.append(device.name)
        return device

    @classmethod
    def from_svdfile(cls, svdfile):
        svd = ET.parse(svdfile)
        return cls.from_svd(svd)

    def refactor_peripheral_instances(self):
        """
        Go through all peripherals and where two have the same RegisterBlock,
        combine them into a single PeripheralPrototype with multiple
        PeripheralInstances.
        """
        to_delete = set()
        to_link = set()
        links = []
        periphs = enumerate(self.peripherals)
        for (idx1, p1), (idx2, p2) in itertools.combinations(periphs, 2):
            if p1 is p2 or idx1 in to_delete or idx2 in to_delete:
                continue
            elif idx1 in to_link or idx2 in to_link:
                continue
            elif p1.registers == p2.registers:
                if p1.name.startswith("tim"):
                    # Similar timers we have to special case, because they
                    # just do not group up well at all.
                    links.append((idx1, idx2))
                    to_link.add(idx2)
                else:
                    # Other peripherals we just move instances together.
                    p1.consume(p2, parent=self)
                    to_delete.add(idx2)
        for idx1, idx2 in links:
            p1 = self.peripherals[idx1]
            p2 = self.peripherals[idx2]
            path = f"super::{p1.name}"
            plink = PeripheralPrototypeLink.from_peripherals(p1, p2, path)
            self.peripherals[idx2] = plink
        for idx in sorted(to_delete, reverse=True):
            del self.peripherals[idx]


class Family(Node):
    """
    Represents a group of devices in a common family.

    Has a name.
    Contains one or more child Devices.
    """
    def __init__(self, name):
        self.name = name
        self.devices = []
        self.peripherals = []

    def to_dict(self):
        return {"name": self.name,
                "devices": [x.to_dict() for x in self.devices]}

    def to_files(self, path, pool):
        familypath = os.path.join(path, self.name)
        os.makedirs(familypath, exist_ok=True)
        periphpath = os.path.join(familypath, "peripherals")
        os.makedirs(periphpath, exist_ok=True)
        pool_results = []
        with open(os.path.join(familypath, "mod.rs"), "w") as f:
            uname = self.name.upper()
            f.write(f"//! Parent module for all {uname} devices.\n\n")
            f.write("/// Peripherals shared by multiple devices\n")
            f.write('pub mod peripherals;\n\n')
            for device in self.devices:
                dname = device.name
                result = pool.apply_async(device.to_files, (familypath,))
                pool_results.append(result)
                f.write(f'#[cfg(any(feature="{dname}", feature="doc"))]\n')
                f.write(f'pub mod {dname};\n\n')
        with open(os.path.join(periphpath, "mod.rs"), "w") as f:
            for peripheral in self.peripherals:
                r = pool.apply_async(peripheral.to_rust_file, (periphpath,))
                pool_results.append(r)
                features = ", ".join(
                    f'feature="{d}"' for d in peripheral.parent_device_names)
                f.write(f'#[cfg(any(feature="doc", {features}))]\n')
                f.write(f'pub mod {peripheral.name};\n\n')
        return pool_results

    def _enumerate_peripherals(self):
        peripherals = []
        for didx, device in enumerate(self.devices):
            for pidx, peripheral in enumerate(device.peripherals):
                peripherals.append((didx, pidx, peripheral))
        return peripherals

    def _match_peripherals(self):
        """Gather all pairs of matching peripherals in this family"""
        to_link = set()
        links = dict()
        peripherals = self._enumerate_peripherals()
        for pt1, pt2 in itertools.combinations(peripherals, 2):
            didx1, pidx1, p1 = pt1
            didx2, pidx2, p2 = pt2
            idx1 = (didx1, pidx1)
            idx2 = (didx2, pidx2)
            if p1 is p2 or idx1 in to_link or idx2 in to_link:
                continue
            elif p1.registers == p2.registers:
                to_link.add(idx2)
                if idx1 not in links:
                    links[idx1] = []
                links[idx1].append(idx2)
        return links

    def refactor_common_peripherals(self):
        """
        Find peripherals shared between devices which are identical and
        refactor them into the family-level shared peripherals.
        """
        # Find all pairs of matching peripherals in the family
        links = self._match_peripherals()

        # Determine which peripherals need versioned names
        # (any with multiple peripherals that share the same name).
        pnames = set()
        dupnames = set()
        for idx in links:
            didx, pidx = idx
            p = self.devices[didx].peripherals[pidx]
            if p.name in pnames:
                dupnames.add(p.name)
            pnames.add(p.name)

        # Now create new crate-level peripherals and replace the old ones
        # with links to the new ones
        versions = {}
        for idx in links:
            # Get the primary member of the link group
            didx, pidx = idx
            device = self.devices[didx]
            p = device.peripherals[pidx]
            # Modify the name to gpio_v1, gpio_v2, etc
            name = p.name
            if name in dupnames:
                if name not in versions:
                    versions[name] = 0
                versions[name] += 1
                name = f'{name}_v{versions[name]}'
            # Make a new PeripheralPrototype for the family, with no instances
            familyp = PeripheralPrototype(name, p.desc)
            familyp.registers = p.registers
            familyp.parent_device_names.append(device.name)
            self.peripherals.append(familyp)
            # Make a link for the primary member
            path = f"{self.name}::peripherals::{name}"
            linkp = PeripheralPrototypeLink(p.name, familyp, path)
            linkp.instances = p.instances
            self.devices[didx].peripherals[pidx] = linkp
            # Make a link for each other member
            for childidx in links[idx]:
                cdidx, cpidx = childidx
                childd = self.devices[cdidx]
                childp = childd.peripherals[cpidx]
                familyp.parent_device_names.append(childd.name)
                linkp = PeripheralPrototypeLink(childp.name, familyp, path)
                linkp.instances = childp.instances
                childd.peripherals[cpidx] = linkp


class Crate:
    """
    Represents the overall crate of devices and shared peripherals.

    Contains one or more child Families and shared Peripherals.
    """
    def __init__(self):
        self.families = []
        self.peripherals = []

    def to_dict(self):
        return {"families": [x.to_dict() for x in self.families],
                "peripherals": [x.to_dict() for x in self.peripherals]}

    def write_build_script(self, path):
        """
        Generates build.rs which copies the relevant device.x into the build
        path for the selected device.
        """
        devices = []
        for family in self.families:
            for device in family.devices:
                if not device.special:
                    devices.append((family.name, device.name))
        clauses = " else ".join("""\
            if env::var_os("CARGO_FEATURE_{}").is_some() {{
                "src/{}/{}/device.x"
            }}""".format(d.upper(), f, d) for (f, d) in sorted(devices))
        clauses += " else { panic!(\"No device features selected\"); }"
        fname = os.path.join(path, "build.rs")
        with open(fname, "w") as f:
            f.write(BUILD_RS_TEMPLATE.format(device_clauses=clauses))
        rustfmt(fname)

    def to_files(self, path, pool):
        """
        Writes src/lib.rs, Cargo.toml, src/mod.rs, build.rs, writes out all
        child peripherals, and triggers all child families to write their own
        files out.
        """
        srcpath = os.path.join(path, 'src')
        if not os.path.isdir(srcpath):
            raise ValueError(f"{srcpath} does not exist")
        periphpath = os.path.join(srcpath, "peripherals")
        os.makedirs(periphpath, exist_ok=True)

        lib_f = open(os.path.join(srcpath, "lib.rs"), "w")
        lib_f.write(CRATE_LIB_PREAMBLE)

        cargo_f = open(os.path.join(path, "Cargo.toml"), "w")
        cargo_f.write(CRATE_CARGO_TOML_PREAMBLE)

        self.write_build_script(path)

        periph_f = open(os.path.join(periphpath, "mod.rs"), "w")

        pool_results = []
        for family in self.families:
            fname = family.name
            pool_results += family.to_files(srcpath, pool)
            features = [f'feature="{d.name}"' for d in family.devices]
            lib_f.write(f'#[cfg(any(feature="doc", {", ".join(features)}))]\n')
            lib_f.write(f'pub mod {fname};\n\n')
            for device in family.devices:
                dname = device.name
                arch = device.cpu.get_architecture().lower().replace("-", "_")
                if device.special:
                    cargo_f.write(f'{dname} = []\n')
                else:
                    cargo_f.write(f'{dname} = ["{arch}"]\n')
                lib_f.write(f'#[cfg(feature="{dname}")]\n')
                lib_f.write(f'pub use {fname}::{dname}::*;\n\n')
        if self.peripherals:
            lib_f.write("//! Peripherals shared between multiple families\n")
            lib_f.write("pub mod peripherals;\n\n")
        for peripheral in self.peripherals:
            result = pool.apply_async(peripheral.to_rust_file, (periphpath,))
            pool_results.append(result)
            features = ", ".join(
                f'feature="{d}"' for d in peripheral.parent_device_names)
            periph_f.write(f'#[cfg(any(feature="doc", {features}))]\n')
            periph_f.write(f'pub mod {peripheral.name};\n\n')
        return pool_results


def get_int(node, tag, default=None):
    """Parses and returns an integer from the specified child tag in node."""
    text = get_string(node, tag, default=default)
    if text == default:
        return text
    text = text.lower().strip()
    if text == "true":
        return 1
    elif text == "false":
        return 0
    elif text[:2] == "0x":
        return int(text[2:], 16)
    elif text[:2] == "0b":
        return int(text[2:], 2)
    else:
        # Annoyingly sometimes constants are base-10 with leading zeros,
        # so int(text, 0) for autodetection does not work.
        return int(text, 10)


def get_string(node, tag, default=None):
    """Finds and returns a string from the specified child tag in node."""
    text = node.findtext(tag, default=default)
    if text == default:
        return text
    return " ".join(text.split())


def rustfmt(fname):
    """Runs rustfmt over the given filename."""
    subprocess.run(["rustfmt", fname])


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("cratepath", help="Path to crate root")
    parser.add_argument("svdfiles", nargs="+", help="SVD files to parse")
    return parser.parse_args()


def main():
    args = parse_args()
    crate = Crate()

    print("Parsing input files...")
    with multiprocessing.Pool() as p:
        devices = p.map(Device.from_svdfile, args.svdfiles)

    print("Collating families...")
    cortex_family = Family("cortex_m")
    crate.families.append(cortex_family)
    for device in devices:
        # Special case the ARMv*-M SVDs
        if device.name.startswith("armv"):
            device.special = True
            cortex_family.devices.append(device)
        else:
            device_family = device.name[:7].lower()
            if device_family not in [f.name for f in crate.families]:
                crate.families.append(Family(device_family))
            family = [f for f in crate.families if f.name == device_family][0]
            family.devices.append(device)

    print("Running refactors...")
    for device in devices:
        device.refactor_peripheral_instances()
        for peripheral in device.peripherals:
            peripheral.refactor_aliased_registers()
            peripheral.refactor_common_register_fields()
            for register in peripheral.registers:
                register.refactor_common_field_values()

    for family in crate.families:
        family.refactor_common_peripherals()

    print("Outputting crate...")
    pool_results = []
    with multiprocessing.Pool() as pool:
        pool_results += crate.to_files(args.cratepath, pool)
        for result in pool_results:
            result.get()


if __name__ == "__main__":
    main()
