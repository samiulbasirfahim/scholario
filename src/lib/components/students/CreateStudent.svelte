<script lang="ts">
	import Icon from '@iconify/svelte';
	import Guardians from './Guardians.svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { onMount } from 'svelte';

	type Guardian = {
		id: number;
		name: string;
		relation: string;
		phone: string;
		address: string;
		photo: string;
	};

	let guardians: Guardian[] = $state([
		{
			id: 1,
			name: 'Abdul Karim',
			relation: 'Father',
			phone: '01711223344',
			address: 'Mymensingh Sadar',
			photo: 'https://i.pravatar.cc/150?img=1'
		},
		{
			id: 2,
			name: 'Rahima Begum',
			relation: 'Mother',
			phone: '01755667788',
			address: 'Muktagacha',
			photo: 'https://i.pravatar.cc/150?img=2'
		},
		{
			id: 3,
			name: 'Selim Hossain',
			relation: 'Uncle',
			phone: '01899887766',
			address: 'Fulbaria',
			photo: 'https://i.pravatar.cc/150?img=3'
		}
	]);

	function removeGuardian(id: number) {
		guardians = guardians.filter((guardian) => guardian.id !== id);
	}

	let fetched_data = {
		religions: ['islam', 'hindu', 'christian']
	};

	// Using your $state approach for form data
	let form_data = $state({
		name: '',
		class_id: '',
		section_id: '',
		dob: '',
		gender: '',
		religion: '',
		address: '',
		phone: '',
		photo: '',
		is_resident: false
	});

	// Handle file input and store the base64 image string
	function handleFileUpload(event: Event) {
		const input = event.target as HTMLInputElement;
		const file = input.files ? input.files[0] : null;

		if (file && file.type.startsWith('image/')) {
			const reader = new FileReader();
			reader.onload = function (e) {
				const base64String = e.target?.result as string;
				form_data.photo = base64String;
			};
			reader.readAsDataURL(file);
		} else {
			alert('Please select a valid image file.');
		}
	}

	$effect(() => {
		console.log($state.snapshot(form_data));
		console.log(form_data.class_id);
	});

	function openGuardiansModal() {
		const studentModal = document.getElementById('create-student-modal') as HTMLDialogElement;
		const guardianModal = document.getElementById('manage-guardians-modal') as HTMLDialogElement;

		studentModal.close();
		guardianModal.showModal();
	}

	onMount(() => {
		classes.fetch();
		sections.fetch();
	});
</script>

<dialog id="create-student-modal" class="modal">
	<div class="modal-box w-11/12 max-w-4xl">
		<form method="dialog">
			<button class="btn btn-sm btn-circle btn-ghost absolute top-2 right-2">✕</button>
		</form>
		<form>
			<h3 class="text-lg font-bold mb-6">Create Student</h3>

			{#if classes.data.length == 0}
				<p class="alert-error alert">Please create a class first</p>
			{:else}
				<div class="grid grid-cols-2 gap-4">
					<div>
						<label for="name" class="block text-sm font-medium">Name</label>
						<input
							type="text"
							id="name"
							name="name"
							class="input input-bordered w-full"
							placeholder="Enter Student Name"
							required
							bind:value={form_data.name}
						/>
					</div>

					<div>
						<label for="class" class="block text-sm font-medium">Class</label>
						<select
							id="class"
							name="class"
							class="input input-bordered w-full"
							required
							bind:value={form_data.class_id}
						>
							<option value="" selected>Class</option>
							{#each classes.data as cls, i (i)}
								<option value={cls.id}>{cls.name}</option>
							{/each}
						</select>
					</div>

					{#if sections.get_by_class(Number(form_data.class_id)).length > 0}
						<div>
							<label for="section" class="block text-sm font-medium">Section</label>
							<select
								id="section"
								name="section"
								class="input input-bordered w-full"
								required
								bind:value={form_data.section_id}
							>
								<option value="" selected>Section</option>
								{#each sections.get_by_class(Number(form_data.class_id)) as section, i (i)}
									<option value={section.id}>{section.name}</option>
								{/each}
							</select>
						</div>
					{/if}

					<div>
						<label for="dob" class="block text-sm font-medium">Date of Birth</label>
						<input
							type="date"
							id="dob"
							name="dob"
							class="input input-bordered w-full"
							required
							bind:value={form_data.dob}
						/>
					</div>

					<div>
						<label for="gender" class="block text-sm font-medium">Gender</label>
						<select
							id="gender"
							name="gender"
							class="input input-bordered w-full"
							required
							bind:value={form_data.gender}
						>
							<option value="" selected>Gender</option>
							<option value="Male">Male</option>
							<option value="Female">Female</option>
						</select>
					</div>

					<div>
						<label for="religion" class="block text-sm font-medium">Religion</label>
						<input
							type="text"
							class="input input-bordered w-full"
							placeholder="Religion"
							list="religions"
							required
							bind:value={form_data.religion}
						/>
						<datalist id="religions">
							{#each fetched_data.religions as religion, i (i)}
								<option value={religion}>{religion}</option>
							{/each}
						</datalist>
					</div>

					<div>
						<label for="address" class="block text-sm font-medium">Address</label>
						<input
							type="text"
							id="address"
							name="address"
							class="input input-bordered w-full"
							placeholder="Enter Address"
							required
							bind:value={form_data.address}
						/>
					</div>

					<div>
						<label for="phone" class="block text-sm font-medium">Phone</label>
						<label class="input validator w-full">
							+880
							<input
								type="tel"
								class="w-full tabular-nums"
								required
								pattern="[0-9]*"
								minlength="10"
								maxlength="10"
								title="Must be 10 digits"
								bind:value={form_data.phone}
							/>
						</label>
					</div>

					<div>
						<label for="photo" class="block text-sm font-medium">Photo</label>
						<input
							type="file"
							class="file-input input-bordered w-full"
							accept="image/*"
							onchange={handleFileUpload}
						/>
					</div>

					<div>
						<label for="is_resident" class="block text-sm font-medium">Resident</label>
						<input
							type="checkbox"
							id="is_resident"
							name="is_resident"
							class="checkbox"
							bind:checked={form_data.is_resident}
						/>
					</div>

					{#if guardians.length > 0}
						<div class=" bg-base-200 col-span-2 rounded p-4">
							<p class="text-xs tracking-wide opacity-60">Guardians</p>
							<ul class="list grid max-h-48 grid-cols-2 gap-4 overflow-y-auto">
								{#each guardians as guardian, i (i)}
									<li
										class="list-row bg-base-300 rounded-box flex items-center justify-between gap-4 p-2"
									>
										<div class="shrink-0">
											<img
												class="size-12 rounded-full object-cover"
												src={guardian.photo}
												alt="{guardian.name}'s photo"
											/>
										</div>

										<div class="min-w-0 flex-1">
											<div class="truncate font-medium">{guardian.name}</div>
											<div class="truncate text-sm text-gray-500">
												{guardian.relation} • {guardian.phone}
											</div>
											<div class="truncate text-sm text-gray-400">{guardian.address}</div>
										</div>

										<button
											class="btn btn-square btn-ghost"
											onclick={(e) => {
												e.preventDefault();
												removeGuardian(guardian.id);
											}}
										>
											<Icon icon="fa:remove" />
										</button>
									</li>
								{/each}
							</ul>
						</div>
					{:else}
						<div class="alert alert-info col-span-2">
							<span>You haven't selected any guardians</span>
						</div>
					{/if}
				</div>
				<div class="modal-action">
					<button
						class="btn btn-secondary"
						onclick={(e) => {
							e.preventDefault();
							openGuardiansModal();
						}}
					>
						Manage Guardians
					</button>
					<button type="submit" class="btn btn-primary">Create Student</button>
				</div>
			{/if}
		</form>
	</div>
</dialog>

<Guardians bind:guardians />
