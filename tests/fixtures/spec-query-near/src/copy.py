def copy(a, b):
    src = open(a)
    dst = open(b, "w")
    dst.write(src.read())
    dst.close()
    src.close()
