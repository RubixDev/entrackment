<script lang="ts">
    import Button, { Icon, Label } from '@smui/button'

    export let expanded: boolean
    export let editDialogOpen: boolean
</script>

<span class="spaced-list">
    <Button on:click={() => (expanded = !expanded)} variant="outlined">
        <Label>{expanded ? 'minimize' : 'expand'}</Label>
        <Icon class="material-icons">{expanded ? 'expand_less' : 'expand_more'}</Icon>
    </Button>
    <Button variant="outlined" on:click={() => editDialogOpen = true}>
        <Label>edit</Label>
        <Icon class="material-icons">edit</Icon>
    </Button>
</span>
