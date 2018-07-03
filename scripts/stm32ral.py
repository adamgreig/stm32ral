#!/usr/bin/env python3
"""
stm32ral.py
Copyright 2018 Adam Greig
Licensed under MIT and Apache 2.0, see LICENSE_MIT and LICENSE_APACHE.
"""

import os
import argparse
import subprocess
import multiprocessing
import xml.etree.ElementTree as ET


CRATE_LIB_PREAMBLE = """
// Copyright 2018 Adam Greig
// See LICENSE-APACHE and LICENSE-MIT for license details.

//! This project provides a register access layer (RAL) for all
//! STM32 microcontrollers.

#![no_std]

#[macro_use]
mod register;

pub use register::{RORegister, WORegister, RWRegister};
pub use register::{UnsafeRORegister, UnsafeRWRegister, UnsafeWORegister};

"""


CRATE_CARGO_TOML_PREAMBLE = """
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

[features]
default = []
unsafe = []
doc = []
"""


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


class Field(Node):
    """
    Represents a field in a register.

    Has a name, description, width, offset, and access.
    Belongs to a parent Register.
    May contain one or more child EnumeratedValues.
    """
    def __init__(self, name, desc, width, offset, access):
        self.name = name
        self.desc = desc
        self.width = width
        self.offset = offset
        self.access = access
        self.r_values = []
        self.w_values = []
        self.rw_values = []

    def to_dict(self):
        return {"name": self.name, "desc": self.desc, "width": self.width,
                "offset": self.offset, "access": self.access,
                "r_values": [x.to_dict() for x in self.r_values],
                "w_values": [x.to_dict() for x in self.w_values],
                "rw_values": [x.to_dict() for x in self.rw_values]}

    def to_rust(self):
        mask = 2**self.width - 1
        rwvals = "\n".join(v.to_rust(self.width) for v in self.rw_values)
        rvals = "\n".join(v.to_rust(self.width) for v in self.r_values)
        wvals = "\n".join(v.to_rust(self.width) for v in self.w_values)
        return f"""
        /// {self.desc}
        pub mod {self.name} {{
            pub const _offset: u32 = {self.offset};
            pub const _mask: u32 = 0b{mask:b};
            {rwvals}

            pub mod read {{
                {rvals}
            }}

            pub mod write {{
                {wvals}
            }}
        }}"""

    @classmethod
    def from_svd(cls, svd, node, ctx):
        ctx = ctx.inherit(node)
        name = get_string(node, 'name')
        desc = get_string(node, 'description')
        width = get_int(node, 'bitWidth')
        offset = get_int(node, 'bitOffset')
        access = ctx.access
        field = cls(name, desc, width, offset, access)
        for evs in node.findall('enumeratedValues'):
            if 'derivedFrom' in evs.attrib:
                df = evs.attrib['derivedFrom']
                evs = svd.find(f".//enumeratedValues[name='{df}']")
                if evs is None:
                    raise ValueError(f"Can't find derivedFrom {df}")
            usage = get_string(evs, 'usage')
            for ev in evs.findall('enumeratedValue'):
                if usage == "read":
                    field.r_values.append(EnumeratedValue.from_svd(svd, ev))
                elif usage == "write":
                    field.w_values.append(EnumeratedValue.from_svd(svd, ev))
                else:
                    field.rw_values.append(EnumeratedValue.from_svd(svd, ev))
        return field


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
        return cls(None, None, None, None)

    def copy(self):
        return RegisterCtx(self.size, self.access, self.reset_value,
                           self.reset_mask)

    def update_from_node(self, node):
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
        fields = "\n".join(f.to_rust() for f in self.fields)
        return f"""
        /// {self.desc}
        pub mod {self.name} {{
            {fields}
        }}"""

    def to_rust_struct_entry(self):
        regtype = {"read-only": "RORegister", "write-only": "WORegister",
                   "read-write": "RWRegister"}[self.access]
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


class PeripheralInstance(Node):
    """
    Represents a specific peripheral instance in a device.

    For example, GPIOA is a PeripheralInstance, while GPIO is a
    PeripheralPrototype which would contain GPIOA. For many
    peripherals there is a single PeripheralPrototype containing
    a single PeripheralInstance.

    Has a name  and base address.
    Belongs to a parent PeripheralPrototype.
    """
    def __init__(self, name, addr):
        self.name = name
        self.addr = addr

    def to_dict(self):
        return {"name": self.name, "addr": self.addr}

    def to_rust(self):
        tname = self.name.title().replace("_", "")
        return f"""
        pub struct {tname} {{}}
        impl ::core::ops::Deref for {tname} {{
            type Target = RegisterBlock;
            fn deref(&self) -> &RegisterBlock {{
                unsafe {{ &*(0x{self.addr:08x} as *const _) }}
            }}
        }}
        pub const {self.name.upper()}: {tname} = {tname} {{}};"""


class PeripheralPrototype(Node):
    """
    Represents a generic peripheral with registers.

    Has a name and description.
    Belongs to a parent Device.
    Contains child PeripheralInstances and Registers.
    """
    def __init__(self, name, desc):
        self.name = name.lower()
        self.desc = desc
        self.registers = []
        self.instances = []

    def to_dict(self):
        return {"name": self.name, "desc": self.desc,
                "registers": [x.to_dict() for x in self.registers],
                "instances": [x.to_dict() for x in self.instances]}

    def to_rust_register_block(self):
        lines = []
        address = 0
        reservedctr = 1
        for register in self.registers:
            if register.offset < address:
                # Aliasing
                continue
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

    def to_rust_file(self, path):
        register_accesses = [r.access for r in self.registers]
        use_registers = []
        if "read-write" in register_accesses:
            use_registers.append("RWRegister")
        if "read-only" in register_accesses:
            use_registers.append("RORegister")
        if "write-only" in register_accesses:
            use_registers.append("WORegister")
        use_registers = ", ".join(use_registers)
        preamble = "\n".join([
            "#![allow(non_snake_case, non_upper_case_globals)]",
            f"use {{{use_registers}}};",
            "",
        ])
        modules = "\n".join(r.to_rust_mod() for r in self.registers)
        instances = "\n".join(i.to_rust() for i in self.instances)
        fname = os.path.join(path, f"{self.name}.rs")
        with open(fname, "w") as f:
            f.write(preamble)
            f.write(modules)
            f.write(self.to_rust_register_block())
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
            print(df)
            raise ValueError(f"No registers found for peripheral {name}")
        ctx = register_ctx
        for register in registers.findall('register'):
            peripheral.registers.append(Register.from_svd(svd, register, ctx))
        peripheral.instances.append(PeripheralInstance(name, addr))
        return peripheral


class CPU(Node):
    """
    Represents the CPU in a device.

    Has a name and nvicPrioBits.
    Belongs to a parent Device.
    """
    def __init__(self, name, nvic_prio_bits):
        self.name = name
        self.nvic_prio_bits = nvic_prio_bits

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
        desc = get_string(node, 'desc')
        value = get_int(node, 'value')
        return cls(name, desc, value)


class Device(Node):
    """
    Represents a device corresponding to a single input SVD file.

    Has a name.
    Contains a child CPU, PeripheralPrototypes, and Interrupts.
    """
    def __init__(self, name, cpu):
        self.name = name.lower()
        self.cpu = cpu
        self.peripherals = []
        self.interrupts = []

    def to_dict(self):
        return {"name": self.name, "cpu": self.cpu.to_dict(),
                "peripherals": [x.to_dict() for x in self.peripherals],
                "interrupts": [x.to_dict() for x in self.interrupts]}

    def to_files(self, familypath):
        devicepath = os.path.join(familypath, self.name)
        os.makedirs(devicepath, exist_ok=True)
        for peripheral in self.peripherals:
            peripheral.to_rust_file(devicepath)
        mod = "\n".join(f"pub mod {p.name};" for p in self.peripherals)
        fname = os.path.join(devicepath, "mod.rs")
        with open(fname, "w") as f:
            f.write(mod+"\n")
        rustfmt(fname)

    @classmethod
    def from_svd(cls, svd):
        """Load a Device node and children from a parsed SVD XML file."""
        name = get_string(svd, 'name')
        cpu = CPU.from_svd(svd, svd.find('cpu'))
        device = cls(name, cpu)
        register_ctx = RegisterCtx.empty()
        register_ctx = register_ctx.inherit(svd)
        for interrupt in svd.findall('.//interrupt'):
            device.interrupts.append(Interrupt.from_svd(svd, interrupt))
        for peripheral in svd.findall('.//peripheral'):
            device.peripherals.append(
                PeripheralPrototype.from_svd(svd, peripheral, register_ctx))
        return device

    @classmethod
    def from_svdfile(cls, svdfile):
        svd = ET.parse(svdfile)
        return cls.from_svd(svd)


class Family(Node):
    """
    Represents a group of devices in a common family.

    Has a name.
    Contains one or more child Devices.
    """
    def __init__(self, name):
        self.name = name
        self.devices = []

    def to_dict(self):
        return {"name": self.name,
                "devices": [x.to_dict() for x in self.devices]}

    def to_files(self, path, pool):
        familypath = os.path.join(path, self.name)
        os.makedirs(familypath, exist_ok=True)
        fname = os.path.join(familypath, "mod.rs")
        pool_results = []
        with open(fname, "w") as f:
            for device in self.devices:
                dname = device.name
                result = pool.apply_async(device.to_files, (familypath,))
                pool_results.append(result)
                f.write(f'#[cfg(any(feature="{dname}", feature="doc"))]\n')
                f.write(f'pub mod {dname};\n\n')
        return pool_results


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

    def to_files(self, path, pool):
        srcpath = os.path.join(path, 'src')
        if not os.path.isdir(srcpath):
            raise ValueError(f"{srcpath} does not exist")

        lib_f = open(os.path.join(srcpath, "lib.rs"), "w")
        lib_f.write(CRATE_LIB_PREAMBLE)

        cargo_f = open(os.path.join(path, "Cargo.toml"), "w")
        cargo_f.write(CRATE_CARGO_TOML_PREAMBLE)

        pool_results = []
        for family in self.families:
            pool_results += family.to_files(srcpath, pool)
            for device in family.devices:
                fname = family.name
                dname = device.name
                cargo_f.write(f"{dname} = []\n")
                lib_f.write(f'#[cfg(any(feature="{dname}", feature="doc"))]\n')
                lib_f.write(f'pub mod {fname};\n')
                lib_f.write(f'#[cfg(feature="{dname}")]\n')
                lib_f.write(f'pub use {fname}::{dname}::*;\n\n')
        return pool_results


def get_int(node, tag, default=None):
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
    text = node.findtext(tag, default=default)
    if text == default:
        return text
    return " ".join(text.split())


def rustfmt(fname):
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
    for device in devices:
        device_family = device.name[:7].lower()
        if device_family not in [f.name for f in crate.families]:
            crate.families.append(Family(device_family))
        family = [f for f in crate.families if f.name == device_family][0]
        family.devices.append(device)
    print("Outputting crate...")
    pool_results = []
    with multiprocessing.Pool() as pool:
        pool_results += crate.to_files(args.cratepath, pool)
        for result in pool_results:
            result.get()
        pool.close()
        pool.join()


if __name__ == "__main__":
    main()
