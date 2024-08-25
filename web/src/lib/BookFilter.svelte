<script lang="ts">
    import Textfield from '@smui/textfield'
    import Icon from '@smui/textfield/icon'
    import Select, { Option } from '@smui/select'
    import SelectIcon from '@smui/select/icon'
    import SegmentedButton, { Segment, Label } from '@smui/segmented-button'

    import { filteredBooks, allBooks, fetching } from '../stores'
    import TagSelection, { matches, TagsQuantifier } from './filter/TagSelection.svelte'

    $: $allBooks, search, tags, tagsQuantifier, filter(), sort()
    $: sortBy, sortDirection, sort()

    let search = ''
    let tags = new Set<number>()
    let tagsQuantifier = TagsQuantifier.AllOf
    let sortBy = 'Title'
    let sortDirection: 'ascending' | 'descending' = 'ascending'

    function filter() {
        $filteredBooks = $allBooks.filter(
            m =>
                m.title.toLowerCase().includes(search.toLowerCase()) &&
                matches(m.tags, tagsQuantifier, tags),
        )
    }

    function sort() {
        // first sort by title so "equal" elements will still be sorted by title
        $filteredBooks.sort((a, b) => a.title.localeCompare(b.title))
        $filteredBooks.sort((_a, _b) => {
            const a = sortDirection === 'ascending' ? _a : _b
            const b = sortDirection === 'ascending' ? _b : _a
            if (sortBy === 'Last Read') {
                const aDate =
                    a.readings.length === 0
                        ? '0000-00-00'
                        : Object.keys(a.readings[0].pages_read).sort().at(-1) || '0000-00-00'
                const bDate =
                    b.readings.length === 0
                        ? '0000-00-00'
                        : Object.keys(b.readings[0].pages_read).sort().at(-1) || '0000-00-00'
                return aDate.localeCompare(bDate)
            } else {
                return a.title.localeCompare(b.title)
            }
        })
        $filteredBooks = $filteredBooks
    }
</script>

<div id="filter" class="card mdc-elevation--z4">
    <h3>Filter</h3>
    <Textfield
        bind:value={search}
        bind:disabled={$fetching}
        label="Search by Title"
        variant="outlined"
        type="search"
    >
        <Icon class="material-icons" slot="leadingIcon">search</Icon>
    </Textfield>

    <!-- <div id="switches"> -->
    <!--     <FormField> -->
    <!--         <Switch bind:checked={showSeen} /> -->
    <!--         <span slot="label">Show seen</span> -->
    <!--     </FormField> -->
    <!-- </div> -->

    <div class="tags">
        <span>Tags:</span>
        <TagSelection bind:selected={tags} bind:quantifier={tagsQuantifier} />
    </div>

    <div class="spaced-list" style="gap: 1rem;">
        <Select bind:value={sortBy} label="Sort by">
            <SelectIcon class="material-icons" slot="leadingIcon">sort</SelectIcon>
            {#each ['Title', 'Last Read'] as sortCriterion}
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

    <span class="hint">{$filteredBooks.length} results</span>
</div>

<style lang="scss">
    #filter {
        display: flex;
        gap: 1rem;
        flex-direction: column;

        h3 {
            margin: 0;
        }

        /* #switches { */
        /*     display: flex; */
        /*     flex-wrap: wrap; */
        /*     gap: 1rem; */
        /* } */

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
