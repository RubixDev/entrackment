<script lang="ts">
    import Switch from '@smui/switch'
    import FormField from '@smui/form-field'
    import Textfield from '@smui/textfield'
    import Icon from '@smui/textfield/icon'
    import Select, { Option } from '@smui/select'
    import SelectIcon from '@smui/select/icon'
    import SegmentedButton, { Segment, Label } from '@smui/segmented-button'

    import {
        filteredMovies,
        allMovies,
        fetching,
        averageOf,
    } from '../stores'
    import TagSelection, { matches, TagsQuantifier } from './filter/TagSelection.svelte'
    import PlatformsSelection from './filter/PlatformsSelection.svelte'

    $: $allMovies,
        search,
        showSeen,
        showUnseen,
        tags,
        tagsQuantifier,
        platforms,
        platformsQuantifier,
        filter(),
        sort()
    $: sortBy, sortDirection, sort()

    let search = ''
    let showSeen = false
    let showUnseen = true
    let tags = new Set<number>()
    let tagsQuantifier = TagsQuantifier.AllOf
    let platforms = new Set<string>()
    let platformsQuantifier = TagsQuantifier.AllOf
    let sortBy = 'Title'
    let sortDirection: 'ascending' | 'descending' = 'ascending'

    function filter() {
        $filteredMovies = $allMovies.filter(
            m =>
                ((showSeen && m.ratings.length !== 0) || (showUnseen && m.ratings.length === 0)) &&
                m.title.toLowerCase().includes(search.toLowerCase()) &&
                matches(m.tags, tagsQuantifier, tags) &&
                matches(m.platforms, platformsQuantifier, platforms),
        )
    }

    function sort() {
        // first sort by title so "equal" elements will still be sorted by title
        $filteredMovies.sort((a, b) => a.title.localeCompare(b.title))
        $filteredMovies.sort((_a, _b) => {
            const a = sortDirection === 'ascending' ? _a : _b
            const b = sortDirection === 'ascending' ? _b : _a
            if (sortBy === 'Rating') {
                return (
                    averageOf(a.ratings.map(r => r.rating)) -
                    averageOf(b.ratings.map(r => r.rating))
                )
            } else if (sortBy === 'Last Watched') {
                const aRating = a.ratings.length === 0 ? { date: '0000-00-00' } : a.ratings[0]
                const bRating = b.ratings.length === 0 ? { date: '0000-00-00' } : b.ratings[0]
                return aRating.date.localeCompare(bRating.date)
            } else if (sortBy === 'Release Date') {
                return a.release_date.localeCompare(b.release_date)
            } else {
                return a.title.localeCompare(b.title)
            }
        })
        $filteredMovies = $filteredMovies
    }
</script>

<div id="filter" class="card mdc-elevation--z4">
    <h3>Filter</h3>
    <Textfield
        bind:value={search}
        bind:disabled={$fetching}
        label="Search by Name"
        variant="outlined"
        type="search"
    >
        <Icon class="material-icons" slot="leadingIcon">search</Icon>
    </Textfield>

    <div id="switches">
        <FormField>
            <Switch bind:checked={showSeen} />
            <span slot="label">Show seen</span>
        </FormField>
        <FormField>
            <Switch bind:checked={showUnseen} />
            <span slot="label">Show unseen</span>
        </FormField>
    </div>

    <div class="tags">
        <!-- TODO: more advanced tag filters, combinable with AND and OR -->
        <span>Tags:</span>
        <TagSelection bind:selected={tags} bind:quantifier={tagsQuantifier} />
    </div>

    <div class="tags">
        <span>Platforms:</span>
        <PlatformsSelection bind:selected={platforms} bind:quantifier={platformsQuantifier} />
    </div>

    <!-- TODO: min/max avg rating -->

    <div class="spaced-list" style="gap: 1rem;">
        <Select bind:value={sortBy} label="Sort by">
            <SelectIcon class="material-icons" slot="leadingIcon">sort</SelectIcon>
            {#each ['Title', 'Rating', 'Last Watched', 'Release Date'] as sortCriterion}
                <Option value={sortCriterion}>{sortCriterion}</Option>
            {/each}
        </Select>
        <SegmentedButton
            segments={['ascending', 'descending']}
            let:segment
            singleSelect
            bind:selected={sortDirection}
        >
            <Segment {segment}>
                <Label>{segment}</Label>
            </Segment>
        </SegmentedButton>
    </div>

    <span class="hint">{$filteredMovies.length} results</span>
</div>

<style lang="scss">
    #filter {
        display: flex;
        gap: 1rem;
        flex-direction: column;

        h3 {
            margin: 0;
        }

        #switches {
            display: flex;
            flex-wrap: wrap;
            gap: 1rem;
        }

        .tags {
            display: flex;
            flex-wrap: wrap;
            gap: 1rem;
            align-items: center;

            & > :first-child {
                min-width: 5rem;
            }
        }
    }
</style>
