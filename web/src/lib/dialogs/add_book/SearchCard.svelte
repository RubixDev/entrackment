<script lang="ts">
    import type { BookEdition, BookStub } from '../../../stores'

    // either icon name or book data
    export let data: string | BookStub | BookEdition
    export let isEdition = false
    export let clickable = false

    let stub: BookStub
    $: stub = data as BookStub
    let edition: BookEdition
    $: edition = data as BookEdition
</script>

<button
    class="card mdc-elevation--z2"
    class:clickable={typeof data !== 'string' || clickable}
    on:click
>
    {#if typeof data === 'string'}
        <div class="img icon-poster">
            <i class="material-icons">{data}</i>
        </div>
        <div class="content"><slot /></div>
    {:else}
        <!-- {#if data.poster !== null} -->
        <img
            src="https://covers.openlibrary.org/b/olid/{isEdition
                ? edition.key.id
                : stub.cover_edition_key}-S.jpg"
            alt=""
            class="img"
        />
        <!-- {:else} -->
        <!--     <div class="img fallback-poster"> -->
        <!--         <i class="material-icons">image_not_supported</i> -->
        <!--     </div> -->
        <!-- {/if} -->
        <div style="overflow: hidden;">
            {#if isEdition}
                <div class="title">
                    <b>{edition.title} ({edition.publish_date})</b>
                </div>
                <div class="desc">ISBN: {[...edition.isbn_13, ...edition.isbn_10].join(', ')}</div>
                <div class="desc">{edition.description || 'no description'}</div>
            {:else}
                <div class="title">
                    <b>{stub.title} ({stub.first_publish_year})</b>
                </div>
                <div class="desc">{stub.author_name.join(', ')}</div>
                <div class="desc">{stub.edition_count} Editions</div>
            {/if}
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

    /* .fallback-poster { */
    /*     background-color: var(--clr-bg-img); */
    /*     display: grid; */
    /*     align-items: center; */
    /*     justify-content: center; */
    /* } */

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
        line-clamp: 1;
    }

    .desc {
        overflow: hidden;
        display: -webkit-box;
        -webkit-line-clamp: 1;
        -webkit-box-orient: vertical;
        line-clamp: 1;
    }
</style>
