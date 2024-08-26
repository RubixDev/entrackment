<script lang="ts">
    import IconButton from '@smui/icon-button'
    import TopAppBar, { Row, Section, Title } from '@smui/top-app-bar'
    import Menu from '@smui/menu'
    import List, { Item, Text, Graphic } from '@smui/list'
    import Dialog, { Content as DialogContent, Title as DialogTitle, Actions } from '@smui/dialog'
    import Button, { Label } from '@smui/button'
    import { colorScheme, SchemeKind } from '../stores'
    import './navbar.scss'

    let menu: Menu
    let confirmDialogOpen = false

    function cycleTheme() {
        $colorScheme = ($colorScheme + 1) % (Object.keys(SchemeKind).length / 2)
    }

    function clearCache() {
        fetch('/api/cache', { method: 'DELETE' })
    }
</script>

<TopAppBar variant="standard">
    <Row>
        <Section>
            <Title>Entrackment</Title>
        </Section>

        <Section id="toolbar" align="end" toolbar>
            <IconButton
                class="material-icons"
                title="Clear Cache"
                on:click={() => (confirmDialogOpen = true)}>delete_forever</IconButton
            >
            <IconButton
                class="material-icons"
                title={$colorScheme === SchemeKind.Light
                    ? 'Light Theme'
                    : $colorScheme === SchemeKind.System
                    ? 'System Theme'
                    : 'Dark Theme'}
                on:click={cycleTheme}
                >{$colorScheme === SchemeKind.Light
                    ? 'light_mode'
                    : $colorScheme === SchemeKind.System
                    ? 'auto_mode'
                    : 'dark_mode'}</IconButton
            >
        </Section>

        <Section id="menubar" align="end" toolbar>
            <IconButton id="menu-button" class="material-icons" on:click={() => menu.setOpen(true)}
                >more_vert</IconButton
            >
            <Menu bind:this={menu}>
                <List>
                    <Item
                        title={$colorScheme === SchemeKind.Light
                            ? 'Light Theme'
                            : $colorScheme === SchemeKind.System
                            ? 'System Theme'
                            : 'Dark Theme'}
                        on:SMUI:action={cycleTheme}
                    >
                        <Graphic class="material-icons"
                            >{$colorScheme === SchemeKind.Light
                                ? 'light_mode'
                                : $colorScheme === SchemeKind.System
                                ? 'auto_mode'
                                : 'dark_mode'}</Graphic
                        >
                        <Text
                            >{$colorScheme === SchemeKind.Light
                                ? 'Switch to Dark Theme'
                                : $colorScheme === SchemeKind.System
                                ? 'Switch to Light Theme'
                                : 'Switch to System Theme'}</Text
                        >
                    </Item>
                    <Item title="Clear Cache" on:SMUI:action={() => (confirmDialogOpen = true)}>
                        <Graphic class="material-icons">delete_forever</Graphic>
                        <Text>Clear Cache</Text>
                    </Item>
                </List>
            </Menu>
        </Section>
    </Row>
</TopAppBar>

<Dialog
    bind:open={confirmDialogOpen}
    aria-labelledby="confirm-dialog-title"
    aria-describedby="confirm-dialog-content"
>
    <DialogTitle id="confirm-dialog-title">Are you sure?</DialogTitle>
    <DialogContent id="confirm-dialog-content"
        >This will irreversibly clear the entire server-side cache.</DialogContent
    >
    <Actions>
        <Button defaultAction><Label>No</Label></Button>
        <Button on:click={clearCache}><Label>Yes</Label></Button>
    </Actions>
</Dialog>
