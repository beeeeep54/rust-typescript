<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<div>
    <pre><code>{JSON.stringify(data, undefined, 2)}</code></pre>
</div>
