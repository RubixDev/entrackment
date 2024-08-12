<script lang="ts">
    export let label: string
    export let color: [number, number, number]
    export let iconLeft: string | null = null
    export let iconLeftSvg: { viewBox: string; path: string } | null = null
    export let iconRight: string | null = null
    export let iconSize: string
</script>

<span class="chip" style:--r={color[0]} style:--g={color[1]} style:--b={color[2]}>
    {#if iconLeft !== null}
        <i class="material-icons" style:font-size={iconSize}>{iconLeft}</i>
    {/if}
    {#if iconLeftSvg !== null}
        <svg
            style:height={iconSize}
            style:width={iconSize}
            fill="currentColor"
            stroke="currentColor"
            stroke-width={0}
            viewBox={iconLeftSvg.viewBox}
            xmlns="http://www.w3.org/2000/svg"
        >
            <path d={iconLeftSvg.path} />
        </svg>
    {/if}
    <span>{label}</span>
    {#if iconRight !== null}
        <i class="material-icons" style:font-size={iconSize}>{iconRight}</i>
    {/if}
</span>

<style lang="scss">
    .chip {
        background-color: rgba(var(--r), var(--g), var(--b), 0.4);
        border: 1px solid rgb(var(--r), var(--g), var(--b));
        border-radius: 2em;
        box-sizing: border-box;
        font-size: 0.75rem;
        line-height: 1.125rem;
        display: inline-flex;
        padding: 0 0.7rem 0 0.5rem;
        align-items: center;
        gap: 0.5rem;
        color: var(--clr-text);
        height: 1.5rem;
    }
</style>
