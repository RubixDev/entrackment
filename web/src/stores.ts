import { type Writable, writable } from 'svelte/store'

export enum SchemeKind {
    Light,
    Dark,
    System,
}
export const colorScheme = writable(SchemeKind.System)
export const darkTheme = writable(false)

export const tags: Writable<{ [index: number]: Tag }> = writable({})
export const allMovies: Writable<Movie[]> = writable([])
export const filteredMovies: Writable<Movie[]> = writable([])
export const allBooks: Writable<Book[]> = writable([])
export const filteredBooks: Writable<Book[]> = writable([])
export const fetching = writable(false)

export const PLATFORMS = [
    'Disney+',
    'Jellyfin',
    'Netflix',
    'Prime Video',
    'YouTube',
    'DVD',
    'BluRay',
    'Cinema',
    'TV',
    'Airplane',
    'Apple TV',
]

export async function fetchApi<T>(request: Promise<Response>, hasBody: boolean = true): Promise<string | T> {
    const response = await request
    if (response.status < 200 || response.status >= 300) {
        const errorText = await response.text()
        return `Server responded with ${response.status} (${response.statusText}): ${errorText}`
    }
    return hasBody ? await response.json() : {}
}

export interface Tag {
    id: number
    name: string
    color: [number, number, number]
    icon: string | null
}

export interface Movie {
    imdb_id: number | null
    tmdb_id: number
    title: string
    description: string
    ratings: Rating[]
    tags: number[]
    platforms: string[]
    poster: string | null
    release_date: string
    runtime: Duration
    score: number
}

export interface MovieStub {
    tmdb_id: number
    title: string
    description: string
    poster: string | null
    release_date: string
}

export interface Rating {
    date: string
    rating: number
    speed: number
    platform: string | null
    tags: number[]
}

export interface Duration {
    secs: number
    nanos: number
}

export interface Book {
    olid: string
    isbn: number
    title: string
    description: string
    authors: string[]
    readings: Reading[]
    tags: number[]
    release_date: string
    start_page: number
    end_page: number
    score: number | null
}

export interface Reading {
    pages_read: Map<Date, number>
    rating: Rating | null
}
