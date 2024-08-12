<script lang="ts">
    export let value = 0
    export let interactive = true
    export let height = '2rem'
    export let className = ''
    export let style = ''
</script>

<div class="root {className}" {style} class:interactive>
    {#each Array(10) as _, i}
        <svg
            on:click={() => { if (interactive) value = i + 1 }}
            on:keydown={event => { if (event.key === 'Enter') value = i + 1 }}
            role="button"
            tabindex="0"
            class:interactive
            style:--height={height}
            fill={i + 1 <= value ? 'var(--clr-yellow)' : 'var(--clr-text-hint)'}
            stroke="currentColor"
            stroke-width={0}
            viewBox={i % 2 === 0 ? '0 0 12 24' : '12 0 12 24'}
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d={i % 2 === 0
                    ? 'M12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21L12 17.27z'
                    : 'M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2z'}
            />
        </svg>
    {/each}
</div>

<style lang="scss">
    .root {
        display: flex;
        filter: drop-shadow(0 0.125rem 0.0625rem #0003);
        &.interactive {
            gap: 0.25rem;
        }
    }

    svg {
        pointer-events: none;
        height: var(--height);
        width: calc(var(--height) / 2);

        &.interactive {
            cursor: pointer;
            pointer-events: auto;
        }
    }
</style>
