<script lang="ts">
    import { tags as allTags, type Tag } from '../../stores'
    import Chip from '../Chip.svelte'

    const unknownTag: Tag = { id: 0, name: 'Unknown', color: [50, 50, 50], icon: 'question_mark' }

    export let tags: number[]
    export let noNoTags = false
</script>

<span class="spaced-list">
    {#each tags.map(id => $allTags[id] || unknownTag) as tag (tag.id)}
        <Chip
            label={tag.name}
            color={tag.color}
            iconLeft="label_outline"
            iconRight={tag.icon}
            iconSize="1.3em"
        />
    {/each}
    {#if tags.length === 0 && !noNoTags}
        no tags
    {/if}
</span>
