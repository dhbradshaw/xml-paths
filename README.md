# xml-paths

If you have a large xml file, it can be useful to try to
understand the structure by getting a map of all the possible element paths.

An easy way to do that is by using this tool.

## Installation

```bash
cargo install xml-paths
```

## Usage

Just run the command and give it a file to parse.

For example, if we have an xml file at `path/to/my/xml_file.xml` that looks like

```xml
<notes>
    <note>
        <to>Gina</to>
        <from>Doug</from>
        <heading>Thanks</heading>
        <body>Lunch was amazing!</body>
    </note>
    <note>
        <to>Doug</to>
        <from>Gina</from>
        <heading>Thanks</heading>
        <body>Thanks Doug! I liked it too -- that's a definite repeat!</body>
    </note>
</notes>
```

,

we run

```bash
xml-paths path/to/my/xml_file.xml
```

to get

```text
/
/notes
/notes/note
/notes/note/body
/notes/note/from
/notes/note/heading
/notes/note/to
```
