# color2filter

Takes a W3C [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/)
color and converts it into a deterministic CSS filter. 

Color results may vary by platform and architecture.
Examples are tested on Ubuntu 24.04.4 LTS x86_64. 

Alpha channel is discarded. All colours are downsampled to sRGB.

Code is derived from [whiskers](https://github.com/ozwaldorf/whiskers/blob/9f1646b95b5f13c087ca63180af2304be5dad39b/src/css_filter.rs).

## Usage

```sh
cargo run --features="cli" --release "<css color>"
```

## Example

### Hex

```sh
cargo run --features="cli" --release "#FF0000"
```

Output:

```css
invert(30%) sepia(46%) saturate(3755%) hue-rotate(342deg) brightness(87%) contrast(134%)
```

### RGB

```sh
cargo run --features="cli" --release "rgb(67 221 254)"
```

Output:

```css
invert(66%) sepia(29%) saturate(3752%) hue-rotate(176deg) brightness(117%) contrast(115%)
```

### OKLCH

```sh
cargo run --features="cli" --release "oklch(0.8244 0.1652 131.03)"
```

Output:

```css
invert(77%) sepia(41%) saturate(546%) hue-rotate(42deg) brightness(99%) contrast(86%)
```