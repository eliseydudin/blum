dependencies = blum

exe = executable(
    'test_map',
    sources: ['testmap.c'],
    link_with: blum,
    include_directories: include_directories('../'),
)
test('test_map', exe)

exe = executable(
    'test_obj',
    sources: ['testobj.c'],
    link_with: blum,
    include_directories: include_directories('../'),
)
test('test_obj', exe)
