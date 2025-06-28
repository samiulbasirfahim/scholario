<script lang="ts">
	const { children } = $props();
</script>

<div class="bg-base-100 z-50 flex rounded p-3 sticky top-4">
	{@render children()}
</div>
