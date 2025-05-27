<script lang="ts">
	const { children } = $props();
</script>

<div class="navbar bg-base-100 z-50 rounded-none px-4 shadow-md">
	{@render children()}
</div>
