export function is_mobile() {
    return /iPhone|iPad|iPod|Android/i.test(navigator.userAgent)
}

export function debounce(fn, ms = 250) {
    let timeout_id
    return function(...args) {
        clearTimeout(timeout_id)
        timeout_id = setTimeout(() => fn.apply(this, args), ms)
    }
}