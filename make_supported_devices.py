import yaml
import argparse


def main():
    parser = argparse.ArgumentParser(
        description="Generate supported_devices.md")
    parser.add_argument(
        "device_table",
        help="Path to stm32_part_table.yaml")
    parser.add_argument(
        "supported_devices",
        help="Path to supported_devices.md")
    args = parser.parse_args()

    with open(args.device_table) as f:
        table = yaml.safe_load(f)

    fout = open(args.supported_devices, "w")
    fout.write("# Supported Devices\n\n")

    for family in table:
        fout.write(f"## {family.upper()}\n\n")
        fout.write("| Feature | Devices | Links |\n")
        fout.write("|:-------:|:-------:|:-----:|\n")
        for device, dt in table[family].items():
            links = "[{}]({}), [st.com]({})".format(
                dt['rm'], dt['rm_url'], dt['url'])
            members = ", ".join(m for m in dt['members'])
            fout.write("| `{}` | {} | {} |\n".format(device, members, links))
        fout.write("\n\n\n")


if __name__ == "__main__":
    main()
