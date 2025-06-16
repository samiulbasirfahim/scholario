<script lang="ts">
	import { toast } from '$lib/store/toast.svelte';
</script>

<div class="toast space-y-3">
	{#each toast.toasts as t, i (i)}
		<div
			class="alert alert-soft alert-{t.type}"
			onclickcapture={() => {
				toast.toasts = toast.toasts.filter((tt) => tt != t);
			}}
		>
			{t.message}
		</div>
	{/each}
</div>
