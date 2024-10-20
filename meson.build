project('blum', 'c')

sources = ['src/blum.c', 'src/map.c', 'src/object.c']

dependencies = []

blum = library('blum', dependencies: dependencies, sources: sources, install: true)

if get_option('test')
    subdir('tests')
endif

pkg = import('pkgconfig')
pkg.generate(blum, description: 'Modern game engine written in C')

install_subdir('blum', install_dir: 'include/blum', strip_directory: true)
