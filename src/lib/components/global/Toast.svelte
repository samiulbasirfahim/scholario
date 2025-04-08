<script lang="ts">
	import { toast } from '$lib/store/toast.svelte';
</script>

<div class="toast space-y-2">
	{#each toast.toasts as tst, i (i)}
		<div
			class="alert alert-soft z-[999999999999999999999] {tst.type === 'warning'
				? 'alert-warning'
				: tst.type === 'error'
					? 'alert-error'
					: 'alert-success'}"
		>
			<span>{tst.message}</span>
		</div>
	{/each}
</div>
