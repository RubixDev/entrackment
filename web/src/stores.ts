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
    'Stan',
]

export async function fetchApi<T>(
    request: Promise<Response>,
    hasBody: boolean = true,
): Promise<string | T> {
    const response = await request
    if (response.status < 200 || response.status >= 300) {
        const errorText = await response.text()
        return `Server responded with ${response.status} (${response.statusText}): ${errorText}`
    }
    return hasBody ? await response.json() : {}
}

export function averageOf(list: number[]): number {
    if (list.length === 0) return 0
    return list.reduce((acc, e) => acc + e, 0) / list.length
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
    isbn: string
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
    pages_read: { [key: string]: number }
    rating: Rating | null
}

export interface BookStub {
    key: Key
    cover_edition_key: string | null
    edition_count: number
    title: string
    author_name: string[]
    first_publish_year: number | null
    ratings_average: number | null
}

export interface Key {
    path: string
    id: string
}

export interface BookEdition {
    key: Key
    isbn_13: string[]
    isbn_10: string[]
    title: string
    description: string | null
    authors: Key[]
    publish_date: string
}
