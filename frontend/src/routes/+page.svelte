<script lang="ts">
  import Content from "$lib/components/Content.svelte";
  import Floater from "$lib/components/Floater.svelte";
  import Navbar from "$lib/components/Navbar.svelte";

  const { data } = $props();
</script>

<Navbar editable={true} />
<Content />
<Floater loadedLanguages={data.loadedLanguages} />
