# usvg-cli

A (rust) tool to process SVG files with usvg (https://github.com/RazrFalcon/resvg/tree/master/crates/usvg).

## From `usvg` README

"SVG is notoriously hard to parse. usvg presents a layer between an XML library and a potential SVG rendering library. It will parse an input SVG into a strongly-typed tree structure were all the elements, attributes, references and other SVG features are already resolved and presented in a simplest way possible. So a caller doesn't have to worry about most of the issues related to SVG parsing and can focus just on the rendering part."

``usvg-cli`` is a simple command-line tool that wraps the usvg parser and allows you to read an SVG file, process it with usvg and then write the processed file. That's it.

## Example Input & Output

Before:

```svg
<!-- https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Basic_Shapes -->
<svg xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 200 250"
    preserveAspectRatio="xMidYMid meet"
    >
  <rect x="10" y="10" width="30" height="30" stroke="black" fill="transparent" stroke-width="5"/>
  <rect x="60" y="10" rx="10" ry="10" width="30" height="30" stroke="black" fill="transparent" stroke-width="5"/>

  <circle cx="25" cy="75" r="20" stroke="red" fill="transparent" stroke-width="5"/>
  <ellipse cx="75" cy="75" rx="20" ry="5" stroke="red" fill="transparent" stroke-width="5"/>

  <line x1="10" x2="50" y1="110" y2="150" stroke="orange" stroke-width="5"/>
  <polyline points="60 110 65 120 70 115 75 130 80 125 85 140 90 135 95 150 100 145"
      stroke="orange" fill="transparent" stroke-width="5"/>

  <polygon points="50 160 55 180 70 180 60 190 65 205 50 195 35 205 40 190 30 180 45 180"
      stroke="green" fill="transparent" stroke-width="5"/>

  <path d="M20,230 Q40,205 50,230 T90,230" fill="none" stroke="blue" stroke-width="5"/>
</svg>
```

After:

```svg
<svg width="200" height="250" viewBox="0 0 200 250" xmlns="http://www.w3.org/2000/svg">
    <defs/>
    <path fill="#000000" fill-opacity="0" stroke="#000000" stroke-width="5" d="M 10 10 L 40 10 L 40 40 L 10 40 Z"/>
    <path fill="#000000" fill-opacity="0" stroke="#000000" stroke-width="5" d="M 70 10 L 80 10 C 85.52285 10 90 14.477153 90 20 L 90 30 C 90 35.522846 85.52285 40 80 40 L 70 40 C 64.47715 40 60 35.522846 60 30 L 60 20 C 60 14.477153 64.47715 10 70 10 Z"/>
    <path fill="#000000" fill-opacity="0" stroke="#ff0000" stroke-width="5" d="M 45 75 C 45 86.04569 36.045696 95 25 95 C 13.954306 95 5 86.04569 5 75 C 5 63.954304 13.954306 55 25 55 C 36.045696 55 45 63.954304 45 75 Z"/>
    <path fill="#000000" fill-opacity="0" stroke="#ff0000" stroke-width="5" d="M 95 75 C 95 77.76142 86.04569 80 75 80 C 63.954304 80 55 77.76142 55 75 C 55 72.23858 63.954304 70 75 70 C 86.04569 70 95 72.23858 95 75 Z"/>
    <path fill="#000000" stroke="#ffa500" stroke-width="5" d="M 10 110 L 50 150"/>
    <path fill="#000000" fill-opacity="0" stroke="#ffa500" stroke-width="5" d="M 60 110 L 65 120 L 70 115 L 75 130 L 80 125 L 85 140 L 90 135 L 95 150 L 100 145"/>
    <path fill="#000000" fill-opacity="0" stroke="#008000" stroke-width="5" d="M 50 160 L 55 180 L 70 180 L 60 190 L 65 205 L 50 195 L 35 205 L 40 190 L 30 180 L 45 180 Z"/>
    <path fill="none" stroke="#0000ff" stroke-width="5" d="M 20 230 Q 40 205 50 230 Q 60 255 90 230"/>
</svg>

```
