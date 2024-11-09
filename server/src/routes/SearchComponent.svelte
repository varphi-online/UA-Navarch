<script lang="ts">

    let department_query: string|null = $state(null);
    let course_number_query: string|null = $state(null);
    let description_query:string|null = $state("");
    let course_limit: number = $state(30);
    // svelte-ignore non_reactive_update
    let previousResults: any[] = [];
    let isLoading = false;
    
    // Create a derived store for the search results
    let searchResults = $derived.by(()=>{
        
        if ((!department_query || department_query.length < 1)&&
            (!course_number_query || course_number_query.length < 1)&&
            (!description_query || description_query.length < 1)) {
            return new Promise<Response>(()=> {});
        }

        let resp = fetch('/api/search', {
            method: 'POST',
            body: JSON.stringify({
                department: department_query,
                course_number:course_number_query,
                desc: description_query,
                limit: course_limit
            }),
            headers: {
                'content-type': 'application/json'
            }
        })
        return resp;
    });
    
    async function search() {
        let resp = await searchResults;
        previousResults = await resp.json();
        return previousResults;
    }

    function highlight(text: string): string {
        const regex = new RegExp(`(${description_query})`, "gi");
        return text.replace(regex, `<mark>$1</mark>`);
    }

    function addLimit(){
        course_limit+=20;
    }
</script>

<div class="search-container">
    <div class="search-box">
        <input 
            type="search"
            placeholder="department"
            bind:value={department_query}
            class="search-input"
        />
        <select bind:value={department_query}>
            <option value="CSC">CSC</option>
        </select>
        <input 
            type="search"
            placeholder="course number"
            bind:value={course_number_query}
            class="search-input"
        />
        <input 
            type="search"
            placeholder="keyword(s)"
            bind:value={description_query}
            class="search-input"
        />
    </div>
        {#await search()}
            {#if department_query.length > 0 || course_number_query.length > 0 || description_query.length > 0}
                <ul class="results-list">
                    {#each previousResults as result}
                        <li class="result-item">
                            <h3>{result.department} {result.course_number} - {@html result.title}</h3>
                            <p>{@html highlight(result.description)}</p>
                        </li>
                    {/each}
                </ul>
            {:else}
                <div class="loading">Search for a class</div>
            {/if}
        {:then results}
            {#if results.length > 0}
                <ul class="results-list">
                    {#each results as result}
                        <li class="result-item">
                            <h3>{result.department} {result.course_number} - {@html result.title}</h3>
                            <p>{@html highlight(result.description)}</p>
                        </li>
                    {/each}
                </ul>
                <button onclick={()=>addLimit()}>Try Load More</button>
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
    .search-box{
        display: flex;
        flex-direction: row;
    }
</style>