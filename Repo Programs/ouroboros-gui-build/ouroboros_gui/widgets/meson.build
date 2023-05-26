pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())
widgetsdir = join_paths(pkgdatadir, 'ouroboros_gui/widgets')

ouroboros_gui_sources = [
    '__init__.py',
    'timezone.py',
    'layout.py',
    'variant.py',
    'desktop.py',
    'disk.py',
    'partition.py',
    'locale.py',
    'selected_locale.py',
]
install_data(ouroboros_gui_sources, install_dir: widgetsdir)
