# color2filter

Takes a W3C [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/)
color and converts it into a deterministic CSS filter. 

Color results may vary by platform and architecture.
Examples are tested on Ubuntu 24.04.4 LTS x86_64. 

Alpha channel is discarded. All colours are downsampled to sRGB.

Code is derived from [whiskers](https://github.com/ozwaldorf/whiskers/blob/9f1646b95b5f13c087ca63180af2304be5dad39b/src/css_filter.rs).

## Usage

```sh
cargo run --release -- "<css color>"
```

## Example

### Hex

```sh
cargo run --release -- "#FF0000"
```

Output:

```css
invert(6%) sepia(11%) saturate(3757%) hue-rotate(360deg) brightness(52%) contrast(111%)
```

### RGB

```sh
cargo run --release -- "rgb(67 221 254)"
```

Output:

```css
invert(0%) sepia(6%) saturate(519%) hue-rotate(282deg) brightness(38%) contrast(100%)
```

### OKLCH

```sh
cargo run --release -- "oklch(0.8244 0.1652 131.03)"
```

Output:

```css
invert(0%) sepia(6%) saturate(519%) hue-rotate(282deg) brightness(38%) contrast(100%)
```