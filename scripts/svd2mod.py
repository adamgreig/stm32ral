#!/usr/bin/env python3
"""
svd2mod.py
Copyright 2018 Adam Greig
Licensed under MIT and Apache 2.0, see LICENSE_MIT and LICENSE_APACHE.

Convert SVD files to Rust modules.
"""

import os
import argparse
import xml.etree.ElementTree as ET
import subprocess
import multiprocessing


def clean_desc(s):
    """Clean a description string by removing consecutive whitespace."""
    return " ".join(s.split())


def iter_node(parent, childtype):
    """
    Iterate all childtype nodes found in parent, if parent is not None.
    """
    if parent is not None:
        for child in parent.iter(childtype):
            yield child


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("svdfiles", nargs="+", help="SVD file to read")
    return parser.parse_args()


def make_field_mod(rtag, ftag):
    """
    Makes the module string containing a field's constants.
    """
    name = ftag.find('name').text
    desc = clean_desc(ftag.find('description').text)
    offset = ftag.find('bitOffset').text
    width = int(ftag.find('bitWidth').text, 0)
    mask = 2**width - 1

    # Handle enumeratedValues
    values = []
    ev = ftag.find('enumeratedValues')
    # Often enumeratedValues are derivedFrom a sibling field
    if ev is not None and 'derivedFrom' in ev.attrib:
        dfname = ev.attrib['derivedFrom']
        dffrom = rtag.findall(f".//enumeratedValues/[name='{dfname}']")
        if dffrom:
            ev = dffrom[0]
        else:
            raise ValueError(f"Can't find ev source for {dfname}")
    for value in iter_node(ev, 'enumeratedValue'):
        values.append((value.find('name').text, value.find('value').text))

    fstr = f"pub const {{name}}: u32 = 0b{{value:0{width}b}};"
    values = "\n".join([fstr.format(name=name, value=int(value, 0))
                        for (name, value) in values])
    module = f"""
    /// {desc}
    pub mod {name} {{
        pub const _offset: u32 = {offset};
        pub const _mask: u32 = 0b{mask:b};"""
    if values:
        module += "\n        " + values
    module += "}"
    return module


def make_register_mod(rtag):
    """
    Makes the module string containing all of a module's field's constants.
    """
    name = rtag.find('name').text.lower()
    desc = clean_desc(rtag.find('description').text)
    fields = []
    for ftag in iter_node(rtag.find('fields'), 'field'):
        fields.append(make_field_mod(rtag, ftag))
    fields = "\n".join(fields)
    module = f"""
    /// {desc}
    pub mod {name} {{
        {fields}
    }}"""
    return module


def make_register_block(ptag):
    """
    Makes the RegisterBlock struct for a peripheral.
    """
    # First find all the registers and assemble into a list for sorting
    registers = []
    for rtag in iter_node(ptag.find('registers'), 'register'):
        name = rtag.find('name').text.lower()
        desc = clean_desc(rtag.find('description').text)
        offset = int(rtag.find('addressOffset').text, 0)
        size_node = rtag.find('size')
        if size_node is None:
            size = 32
        else:
            size = int(size_node.text, 0)
        access = rtag.find('access')
        if access is not None:
            access = access.text
        else:
            access = "read-write"
        registers.append((offset, name, desc, size, access))

    # Sort into offset order
    registers.sort()

    # Now turn each register into an entry in the struct
    lines = []
    address = 0
    reservedctr = 1
    for offset, name, desc, size, access in registers:
        # Detect aliasing. Ugh.
        if offset < address:
            continue
        # Make up gaps between registers
        if offset != address:
            gaps = []
            u32s = (offset - address) // 4
            if u32s != 0:
                gaps.append("[u32; {}]".format(u32s))
                address += u32s * 4
            u16s = (offset - address) // 2
            if u16s != 0:
                gaps.append("[u16; {}]".format(u16s))
                address += u16s * 2
            u8s = (offset - address)
            if u8s != 0:
                gaps.append("[u8; {}]".format(u8s))
                address += u8s
            for gaptype in gaps:
                lines.append("_reserved{}: {},".format(reservedctr, gaptype))
                reservedctr += 1

        if size == 32:
            regwidth = "u32"
        elif size == 16:
            regwidth = "u16"
        elif size == 8:
            regwidth = "u8"
        else:
            raise ValueError("Unhandled register size {}".format(size))

        if access == "read-write":
            regtype = "RWRegister"
        elif access == "read-only":
            regtype = "RORegister"
        elif access == "write-only":
            regtype = "WORegister"
        else:
            raise ValueError("Unhandled register access {}".format(access))

        lines.append(f"/// {desc}\npub {name}: {regtype}<{regwidth}>,")
        address += size // 8

    lines = "\n".join(lines)

    struct = f"""
    pub struct RegisterBlock {{
        {lines}
    }}"""
    return struct


def make_instance(ptag):
    name = ptag.find('name').text
    name = name[0].upper() + name[1:].lower()
    name_u = name.upper()
    address = int(ptag.find('baseAddress').text, 0)
    instance = f"""
    pub struct {name} {{}}
    impl ::core::ops::Deref for {name} {{
        type Target = RegisterBlock;
        fn deref(&self) -> RegisterBlock {{
            unsafe {{ &*(0x{address:08x} as *const _) }}
        }}
    }}
    pub const {name_u}: {name} = {name} {{}};"""

    return instance


def process_device(svdpath):
    print("Processing", svdpath)
    svd = ET.parse(svdpath)
    dname = svd.find('name').text
    try:
        os.mkdir(dname.lower())
    except FileExistsError:
        pass
    pnames = []
    for ptag in iter_node(svd.find('peripherals'), 'peripheral'):
        pname = ptag.find('name').text
        # Skip derivedFrom peripherals here; we find them again later
        if 'derivedFrom' in ptag.attrib:
            continue
        plines = []
        # Make a module for each register (containing one for each field)
        for rtag in iter_node(ptag.find('registers'), 'register'):
            plines.append(make_register_mod(rtag))
        # Make register block struct
        plines.append(make_register_block(ptag))
        # Make struct and impl for the primary instance
        plines.append(make_instance(ptag))
        # Make a struct and impl for each peripheral derivedFrom this one
        for df_ptag in svd.findall(f".//peripheral[@derivedFrom='{pname}']"):
            plines.append(make_instance(df_ptag))
        # Write out
        fname = os.path.join(dname.lower(), pname.lower() + ".rs")
        with open(fname, "w") as f:
            f.write("\n".join(plines))
        subprocess.run(["rustfmt", fname])
        pnames.append(pname.lower())
    modpath = os.path.join(dname.lower(), "mod.rs")
    with open(modpath, "w") as f:
        f.write("\n".join(f"pub mod {n};" for n in pnames))


def main():
    args = parse_args()
    with multiprocessing.Pool() as p:
        p.map(process_device, args.svdfiles)


if __name__ == "__main__":
    main()
