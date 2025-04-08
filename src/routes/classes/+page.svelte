<script lang="ts">
	import CreateClass from '$lib/components/classes/CreateClass.svelte';
	import CreateSection from '$lib/components/classes/CreateSection.svelte';
	import CreateSubject from '$lib/components/classes/CreateSubject.svelte';
	import LinkClassSubject from '$lib/components/classes/LinkClassSubject.svelte';
	import { classes, subjects } from '$lib/store/class.svelte';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';

	let selectedClass: number | null = $state(null);

	onMount(() => {
		classes.fetch();
		subjects.fetch();
	});
</script>

<div class="navbar bg-base-300 rounded px-4">
	<div class="flex-1">
		<p>Classes & Subjects</p>
	</div>
	<div class="flex gap-2">
		<button
			class="btn btn-secondary"
			onclick={() => {
				(document.getElementById('create-subject-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="material-symbols:book" />
			Create Subject
		</button>

		<button
			class="btn btn-accent"
			onclick={() => {
				(document.getElementById('create-section-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="teenyicons:section-add-solid" />
			Create Section
		</button>
		<button
			class="btn btn-primary"
			onclick={() => {
				(document.getElementById('create-class-modal') as HTMLDialogElement).showModal();
			}}
		>
			<Icon icon="gridicons:create" />
			Create Class</button
		>
	</div>
</div>

<ul>
	{#each classes.data as cls, i (i)}
		<li>
			<button
				onclick={() => {
					(document.getElementById('link-subjects-modal') as HTMLDialogElement).showModal();
					selectedClass = cls.id;
				}}>{cls.name}</button
			>
		</li>
	{/each}
</ul>

<CreateClass />
<CreateSection />
<CreateSubject />
<LinkClassSubject {selectedClass} />
