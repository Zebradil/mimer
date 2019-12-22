# mimer

Easily set default applications from command line.

Mimer scans `/usr/share/applications`, extracts mime types and creates
`~/.config/mimeapps.list` with mapping between mime type and application,
which supports that mime type. If there are several candidates for a mime type,
you'll need to choose one of them.
