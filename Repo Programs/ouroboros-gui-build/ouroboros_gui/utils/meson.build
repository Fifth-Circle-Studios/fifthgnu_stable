pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())
utilsdir = join_paths(pkgdatadir, 'ouroboros_gui/utils')

ouroboros_gui_sources = [
    '__init__.py',
    'disks.py',
    'threading.py',
    'command.py',
    'gtk_helpers.py'
]
install_data(ouroboros_gui_sources, install_dir: utilsdir)
