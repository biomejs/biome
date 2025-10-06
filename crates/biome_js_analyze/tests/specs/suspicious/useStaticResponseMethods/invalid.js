// Missing the `Content-Type` header in the response,
// very often this is a mistake so we catch it anyway.
new Response(JSON.stringify({ value: 0 }))

new Response(JSON.stringify({ value: 0 }), {})

new Response(JSON.stringify({ value: 0 }), {
    headers: {}
})

new Response(JSON.stringify({ value: 0 }), {
    headers: {
        'Content-Type': 'application/json',
    }
})

new Response(JSON.stringify({ value: 0 }), {
    headers: {
        'content-type': 'application/json',
    }
})

new Response(null, {
    headers: {
        Location: 'http://example.com',
    },
    status: 301,
})


new Response(undefined, {
    headers: {
        location: 'https://example.com',
    },
    status: 302,
})

new Response('', {
    headers: {
        location: 'https://example.com',
    },
    status: 303,
})