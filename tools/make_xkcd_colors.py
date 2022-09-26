"""
take the colors from https://xkcd.com/color/rgb.txt and turn them into ColorRgbFs
"""

with open("xkcd_color.txt") as f:
    for l in f.readlines():
        l = l.strip()
        s = l.split("\t")
        color_hex = s[0]
        color_name = s[-1]
        color_name = "XKCD_" + color_name.replace(' ', '_')

        red_val = int(color_hex[1:3], 16)
        green_val = int(color_hex[3:5], 16)
        blue_val = int(color_hex[5:], 16)
        
        print("pub const {}:ColorRgbF = ColorRgbF {{ r: {}.0, g: {}.0, b: {}.0 }};".format(color_name, red_val, green_val, blue_val))
