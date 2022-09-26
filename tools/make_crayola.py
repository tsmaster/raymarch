with open("crayola_color.txt") as f:
    for l in f.readlines():
        l = l.strip()
        name, colors = l.split("\t")
        name = name.replace(" ", "_")

        col_components = colors.split(',')
        red = col_components[0].strip()
        green = col_components[1].strip()
        blue = col_components[2].strip()

        print("pub const CRAYOLA_{}:ColorRgbF = ColorRgbF {{ r: {}, g: {} b: {} }};".format(name, red, green, blue))
        
