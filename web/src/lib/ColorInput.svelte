<script lang="ts">
    type Color = [number, number, number]

    export let icon = 'colorize'
    export let value: Color
    export let size = '36px'

    let hexValue = ''
    makeHex()
    $: hexValue, parseHex()
    $: value, makeHex()

    function parseHex() {
        const r = parseInt(hexValue.slice(1, 3), 16)
        const g = parseInt(hexValue.slice(3, 5), 16)
        const b = parseInt(hexValue.slice(5, 7), 16)
        value = [r, g, b]
    }

    function makeHex() {
        hexValue = `#${value[0].toString(16).padStart(2, '0')}${value[1].toString(16).padStart(2, '0')}${value[2].toString(16).padStart(2, '0')}`
    }

    function contrastText(bg: Color): string {
        return (bg[0] * 299 + bg[1] * 587 + bg[2] * 114) / 1000 > 125 ? 'black' : 'white'
    }
</script>

<span class="picker" style:--size={size} style:--icon={icon} style:--icon-color={contrastText(value)}>
    <i class="material-icons icon">{icon}</i>
    <input type="color" bind:value={hexValue}>
</span>

<style lang="scss">
    .picker {
        width: var(--size);
        height: var(--size);
        position: relative;

        input {
            background: none;
            border: none;
            padding: 0;
            cursor: pointer;
            width: 100%;
            height: 100%;

            &::-webkit-color-swatch-wrapper {
                padding: 0;
            }

            &::-webkit-color-swatch {
                border: none;
                border-radius: var(--mdc-shape-small, 4px);
            }
            &::-moz-color-swatch {
                border: none;
                border-radius: var(--mdc-shape-small, 4px);
            }
        }

        .icon {
            position: absolute;
            --mdc-icon-size: calc(var(--size) - 12px);
            color: var(--icon-color);
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            pointer-events: none;
        }
    }
</style>
