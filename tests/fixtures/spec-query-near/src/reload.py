def reload(f, path):
    f.close()
    return open(path)
