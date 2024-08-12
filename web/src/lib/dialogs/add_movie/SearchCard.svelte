<script lang="ts">
    import type { MovieStub } from '../../../stores'

    // either icon name or movie data
    export let data: string | MovieStub
</script>

<button class="card mdc-elevation--z2" class:clickable={typeof data !== 'string'} on:click>
    {#if typeof data === 'string'}
        <div class="img icon-poster">
            <i class="material-icons">{data}</i>
        </div>
        <div class="content"><slot /></div>
    {:else}
        {#if data.poster !== null}
            <img src="/api/posters/small{data.poster}" alt="poster" class="img" />
        {:else}
            <div class="img fallback-poster">
                <i class="material-icons">image_not_supported</i>
            </div>
        {/if}
        <div style="overflow: hidden;">
            <div class="title">
                <b>{data.title} ({new Date(data.release_date).getFullYear()})</b>
            </div>
            <div class="description">{data.description}</div>
        </div>
    {/if}
</button>

<style lang="scss">
    .card {
        // button reset
        border: none;
        margin: 0;
        padding: 0;
        width: auto;
        overflow: visible;
        background: transparent;
        color: inherit;
        font: inherit;
        line-height: normal;
        text-align: inherit;
        -webkit-font-smoothing: inherit;
        -moz-osx-font-smoothing: inherit;

        box-sizing: border-box;
        overflow: hidden;
        padding: 0.5rem 1rem 0.5rem calc(var(--poster-width) + 1rem) !important;
        background-color: var(--clr-bg-card) !important;
        --height: 5rem;
        height: var(--height);
        position: relative;
        line-height: 1.2rem;

        --poster-aspect-ratio: 92 / 138;
        --poster-width: calc(var(--poster-aspect-ratio) * var(--height));

        &.clickable {
            cursor: pointer;
        }
    }

    .img {
        height: var(--height);
        width: var(--poster-width);
        position: absolute;
        top: 0;
        left: 0;
    }

    .icon-poster {
        background-color: var(--clr-primary);
        display: grid;
        align-items: center;
        justify-content: center;
    }

    .content {
        display: flex;
        align-items: center;
        height: 100%;
        gap: 1rem;
    }

    .fallback-poster {
        background-color: var(--clr-bg-img);
        display: grid;
        align-items: center;
        justify-content: center;
    }

    i {
        font-size: 2.5rem;
        color: var(--clr-bg-card);
    }

    .title {
        overflow: hidden;
        display: -webkit-box;
        -webkit-line-clamp: 1;
        -webkit-box-orient: vertical;
        margin-bottom: 0.5rem;
    }

    .description {
        overflow: hidden;
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
    }
</style>
