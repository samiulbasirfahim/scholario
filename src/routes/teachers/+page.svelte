<script lang="ts">
	import InputContainer from '$lib/components/global/InputContainer.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { Teacher } from '$lib/type/teacher';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';

	let teachers: Teacher[] = $state([]);

	let error_message = $state('');

	let teacher_form = $state({
		name: '',
		phone: '',
		address: '',
		salary: '',
		photo: ''
	});

	const add_teacher = () => {
		invoke('teacher_save', {
			...teacher_form,
			salary: Number(teacher_form.salary),
			phone: `+880${teacher_form.phone}`
		})
			.then((d) => {
				teachers.push(d as Teacher);
			})
			.catch((p) => {
				error_message = p as string;
				setTimeout(() => (error_message = ''), 4000);
			});
	};

	const delete_teacher = (id: number) => {
		invoke('teacher_delete', { id })
			.then(() => {
				teachers = teachers.filter((t) => t.id !== id);
			})
			.catch((e) => console.log(e));
	};

	onMount(() => {
		invoke('teacher_get_all')
			.then((d) => (teachers = d as Teacher[]))
			.catch((e) => console.log(e));
	});
</script>

<div class="flex w-full flex-col p-8 lg:flex-row">
	<div class="flex w-full items-start justify-start">
		<ul class="list bg-base-300 rounded-box max-h-[90vh] w-full gap-4 overflow-y-auto p-4">
			{#if teachers.length > 0}
				{#each teachers as teacher, i (i)}
					<li class="list-row bg-base-100 flex items-center justify-between">
						<div class="flex items-center justify-start">
							<div class="avatar">
								<div class="rounded-box w-16">
									<img src={teacher.photo} alt="{teacher.name}'s photo" />
								</div>
							</div>
							<div class="ps-2">
								<div class="text-base font-semibold">{teacher.name}</div>
								<div class="text-sm font-medium uppercase">{teacher.phone}</div>
								<div class="text-xs font-light">{teacher.address}</div>
							</div>
						</div>

						<div class="flex gap-2">
							<button
								class="btn btn-square btn-ghost opacity-40 hover:opacity-100"
								onclick={() => delete_teacher(teacher.id)}
							>
								<Icon icon="material-symbols:delete-rounded" font-size="26" />
							</button>
							<button class="btn btn-square btn-ghost opacity-40 hover:opacity-100">
								<Icon icon="majesticons:open" font-size="26" />
							</button>
						</div>
					</li>
				{/each}
			{:else}
				<div class="alert alert-warning">
					<span>No teacher found</span>
				</div>
			{/if}
		</ul>
	</div>

	<div class="divider lg:divider-horizontal"></div>

	<div class="flex max-h-60 w-full flex-col items-start justify-start gap-4 overflow-y-visible">
		<form
			class="card bg-base-300 rounded-box grid w-full grow grid-cols-1 place-items-center gap-4 p-4 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
			onsubmit={(e) => {
				e.preventDefault();
				add_teacher();
			}}
		>
			<InputContainer name="name" label="Teacher Name: ">
				<input
					required
					type="text"
					name="name"
					class="input w-full"
					bind:value={teacher_form.name}
				/>
			</InputContainer>

			<InputContainer name="phone" label="Phone: ">
				<label class="input">
					+880
					<input
						type="tel"
						class="validator grow tabular-nums"
						required
						pattern="[0-9]*"
						minlength="10"
						maxlength="10"
						title="Must be 10 digits"
						bind:value={teacher_form.phone}
					/>
					<p class="validator-hint">Must be 10 digits</p>
				</label>
			</InputContainer>

			<InputContainer name="address" label="Address: ">
				<input
					required
					type="text"
					name="address"
					class="input w-full"
					bind:value={teacher_form.address}
				/>
			</InputContainer>

			<InputContainer name="salary" label="Salary: ">
				<input
					type="number"
					required
					name="salary"
					class="input w-full"
					bind:value={teacher_form.salary}
				/>
			</InputContainer>

			<InputContainer name="photo" label="Photo: ">
				<input
					type="file"
					required
					class="file-input file-input-primary w-full"
					name="photo"
					accept="image/*"
					onchange={(e) => {
						let files = (e.target as HTMLInputElement).files;
						if (!files) return;

						let file = files[0];

						const reader = new FileReader();

						reader.onload = (ev) => {
							let base64String = ev.target.result.split(',')[1];
							teacher_form.photo = 'data:image/png;base64,' + base64String;
						};

						reader.readAsDataURL(file);
					}}
				/>
			</InputContainer>

			{#if error_message.length > 0}
				<div class="alert alert-error">
					<span>{error_message}</span>
				</div>
			{/if}

			<InputContainer name="submit" label="+">
				<input type="submit" class="btn btn-primary w-full" value="Add Teacher" />
			</InputContainer>
		</form>
	</div>
</div>
