<script lang="ts">
    import { type Duration } from '../../stores'

    export let runtime: Duration

    function formatDuration({ secs, nanos }: Duration): string {
        if (secs === 0 && nanos === 0) {
            return '0s'
        }

        const years = Math.trunc(secs / 31_557_600) // 365.25d
        const ydays = secs % 31_557_600
        const months = Math.trunc(ydays / 2_630_016) // 30.44d
        const mdays = ydays % 2_630_016
        const days = Math.trunc(mdays / 86400)
        const day_secs = mdays % 86400
        const hours = Math.trunc(day_secs / 3600)
        const minutes = Math.trunc((day_secs % 3600) / 60)
        const seconds = day_secs % 60

        const millis = Math.trunc(nanos / 1_000_000)
        const micros = Math.trunc(nanos / 1000) % 1000
        const nanosec = nanos % 1000

        let out = ''
        let started = false
        const item = (name: string, value: number, allowPlural: boolean) => {
            if (value > 0) {
                if (started) out += '\xa0'
                out += value
                out += name
                if (value > 1 && allowPlural) {
                    out += 's'
                }
                started = true
            }
        }

        item('year', years, true)
        item('month', months, true)
        item('day', days, true)
        item('h', hours, false)
        item('m', minutes, false)
        item('s', seconds, false)
        item('ms', millis, false)
        item('µs', micros, false)
        item('ns', nanosec, false)

        return out
    }
</script>

<span>{formatDuration(runtime)}</span>
