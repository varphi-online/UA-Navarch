<script lang="ts">
    
    // Create a writable store for the search query
    let query = $state("");
    
    // Create a derived store for the search results
    let searchResults = $derived.by(()=>{
        if (!query || query.length < 1) {
            return new Promise<Response>(()=> {});
        }
        let resp = fetch('/api/search', {
            method: 'POST',
            body: JSON.stringify({ desc: query }),
            headers: {
                'content-type': 'application/json'
            }
        })
        return resp;
    });
    
    async function search() {
        let resp = await searchResults;
        return resp.json();
    }

    function highlight(text: string): string {
        const regex = new RegExp(`(${query})`, "gi");
        return text.replace(regex, `<mark>$1</mark>`);
    }
</script>

<div class="search-container">
    <input 
        type="text"
        placeholder="Search..."
        bind:value={query}
        class="search-input"
    />
        {#await search()}
            {#if query !== ""}
                <div class="loading">Loading...</div>
            {:else}
                <div class="loading">Search for a class</div>
            {/if}
        {:then results}
            {#if results.length > 0}
                <ul class="results-list">
                    {#each results as result}
                        <li class="result-item">
                            <h3>{result.department} {result.course_number} - {result.title}</h3>
                            <p>{@html highlight(result.description)}</p>
                        </li>
                    {/each}
                </ul>
            {:else}
                <div class="no-results">No results found</div>
            {/if}
        {:catch error}
            <div class="error">Error: {error.message}</div>
        {/await}

</div>

<style>
    mark {
        background-color: yellow;
    }
</style>