pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())
classesdir = join_paths(pkgdatadir, 'ouroboros_gui/classes')

ouroboros_gui_sources = [
    '__init__.py',
    'locale.py',
    'keymap.py',
    'install_prefs.py',
    'partition.py',
    'ouroboros_screen.py'
]
install_data(ouroboros_gui_sources, install_dir: classesdir)
