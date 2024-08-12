<script lang="ts">
    import Switch from '@smui/switch'
    import FormField from '@smui/form-field'
    import Textfield from '@smui/textfield'
    import Icon from '@smui/textfield/icon'

    import { filteredMovies, filteredBooks, allMovies, allBooks, fetching } from '../stores'

    $: $filteredMovies = $allMovies.filter(
        m =>
            ((showSeen && m.ratings.length !== 0) || (showUnseen && m.ratings.length === 0)) &&
            m.title.toLowerCase().includes(search.toLowerCase()),
    )
    $: $filteredBooks = $allBooks.filter(m => m.title.toLowerCase().includes(search.toLowerCase()))

    let showSeen = false
    let showUnseen = true
    let search = ''
</script>

<div id="filter" class="card mdc-elevation--z4">
    <h3>Filter</h3>
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
    <Textfield
        bind:value={search}
        bind:disabled={$fetching}
        label="Search"
        variant="outlined"
        type="search"
    >
        <Icon class="material-icons" slot="leadingIcon">search</Icon>
    </Textfield>
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
            flex-direction: column;
        }
    }
</style>
