<script lang="ts">
	interface Student {
		id: number;
		name: string;
		class_id: number;
		section_id: number;
		session_id: number;
		dob: string;
		gender: string;
		religion: string;
		address: string;
		phone: string | null;
		admission_date: string;
		is_resident: boolean;
		roll: number;
		photo: string | null;
		health_notes: string | null;
		general_notes: string | null;
	}

	interface Guardian {
		id: number;
		name: string;
		phone: string;
		photo: string;
		address: string;
		relationship: string;
	}

	const students: Student[] = [
		{
			id: 1,
			name: 'Fahim Hasan',
			class_id: 10,
			section_id: 1,
			session_id: 2024,
			dob: '2004-07-15',
			gender: 'Male',
			religion: 'Islam',
			address: 'Dhaka, Bangladesh',
			phone: '017XXXXXXXX',
			admission_date: '2021-01-10',
			is_resident: false,
			roll: 12,
			photo: 'https://randomuser.me/api/portraits/men/32.jpg',
			health_notes: 'Has asthma.',
			general_notes: 'Very attentive.'
		},
		{
			id: 2,
			name: 'Ayesha Rahman',
			class_id: 10,
			section_id: 2,
			session_id: 2024,
			dob: '2005-02-20',
			gender: 'Female',
			religion: 'Islam',
			address: 'Chittagong, Bangladesh',
			phone: null,
			admission_date: '2020-05-15',
			is_resident: true,
			roll: 7,
			photo: 'https://randomuser.me/api/portraits/women/44.jpg',
			health_notes: null,
			general_notes: 'Excellent in sports.'
		}
	];

	const guardiansByStudent: Record<number, Guardian[]> = {
		1: [
			{
				id: 1,
				name: 'Hasan Ali',
				phone: '018XXXXXXXX',
				photo: 'https://randomuser.me/api/portraits/men/50.jpg',
				address: 'Dhaka, Bangladesh',
				relationship: 'Father'
			},
			{
				id: 2,
				name: 'Farida Hasan',
				phone: '019XXXXXXXX',
				photo: 'https://randomuser.me/api/portraits/women/50.jpg',
				address: 'Dhaka, Bangladesh',
				relationship: 'Mother'
			}
		],
		2: [
			{
				id: 3,
				name: 'Rahman Ali',
				phone: '017XXXXXXXX',
				photo: 'https://randomuser.me/api/portraits/men/52.jpg',
				address: 'Chittagong, Bangladesh',
				relationship: 'Father'
			}
		]
	};

	let selectedStudentId: number | null = null;

	$: selectedStudent = students.find((s) => s.id === selectedStudentId) ?? null;
	$: guardians = selectedStudentId !== null ? guardiansByStudent[selectedStudentId] || [] : [];
</script>

<div class="flex gap-8">
	<div class="w-2/5">
		<h2 class="mb-4 border-b border-gray-300 pb-2 text-xl font-bold">Students</h2>
		<table class="w-full table-auto border border-gray-300 text-sm">
			<thead>
				<tr class="bg-gray-100">
					<th class="border border-gray-300 p-2">#</th>
					<th class="border border-gray-300 p-2">Photo</th>
					<th class="border border-gray-300 p-2 text-left">Name</th>
					<th class="border border-gray-300 p-2">Class</th>
					<th class="border border-gray-300 p-2">Section</th>
					<th class="border border-gray-300 p-2">Roll</th>
				</tr>
			</thead>
			<tbody>
				{#each students as student, i (student.id)}
					<tr
						on:click={() => (selectedStudentId = student.id)}
						class="cursor-pointer hover:bg-blue-100"
						class:selected={selectedStudentId === student.id}
						class:bg-blue-200={selectedStudentId === student.id}
					>
						<td class="border border-gray-300 p-2 text-center">{i + 1}</td>
						<td class="border border-gray-300 p-2 text-center">
							{#if student.photo}
								<img
									src={student.photo}
									alt="photo"
									class="mx-auto h-10 w-10 rounded-full object-cover"
								/>
							{:else}
								N/A
							{/if}
						</td>
						<td class="border border-gray-300 p-2">{student.name}</td>
						<td class="border border-gray-300 p-2 text-center">{student.class_id}</td>
						<td class="border border-gray-300 p-2 text-center">{student.section_id}</td>
						<td class="border border-gray-300 p-2 text-center">{student.roll}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	<!-- Student Details + Guardians -->
	<div class="bg-base-100 w-3/5 rounded border border-gray-300 p-6 text-gray-700">
		{#if selectedStudent}
			<h2 class="text-primary mb-4 border-b border-gray-400 pb-2 text-2xl font-bold">
				Student Details
			</h2>
			<div class="flex gap-6">
				{#if selectedStudent.photo}
					<img
						src={selectedStudent.photo}
						alt="Student photo"
						class="h-28 w-28 flex-shrink-0 rounded-full object-cover"
					/>
				{/if}

				<div class="grid flex-grow grid-cols-2 gap-6 text-sm">
					<div class="space-y-3">
						<div>
							<p class="text-gray-500">Name</p>
							<p class="font-medium">{selectedStudent.name}</p>
						</div>
						<div>
							<p class="text-gray-500">Gender</p>
							<p class="font-medium">{selectedStudent.gender}</p>
						</div>
						<div>
							<p class="text-gray-500">Date of Birth</p>
							<p class="font-medium">{new Date(selectedStudent.dob).toLocaleDateString()}</p>
						</div>
						<div>
							<p class="text-gray-500">Phone</p>
							<p class="font-medium">{selectedStudent.phone ?? 'N/A'}</p>
						</div>
						<div>
							<p class="text-gray-500">Admission Date</p>
							<p class="font-medium">
								{new Date(selectedStudent.admission_date).toLocaleDateString()}
							</p>
						</div>
						<div class="flex flex-wrap gap-3 pt-2">
							<button class="btn btn-info btn-sm">Edit</button>
							<button class="btn btn-error btn-sm">Delete</button>
						</div>
					</div>

					<div class="space-y-3">
						<div>
							<p class="text-gray-500">Class</p>
							<p class="font-medium">{selectedStudent.class_id}</p>
						</div>
						<div>
							<p class="text-gray-500">Section</p>
							<p class="font-medium">{selectedStudent.section_id}</p>
						</div>
						<div>
							<p class="text-gray-500">Roll</p>
							<p class="font-medium">{selectedStudent.roll}</p>
						</div>
						<div>
							<p class="text-gray-500">Resident</p>
							<p class="font-medium">{selectedStudent.is_resident ? 'Yes' : 'No'}</p>
						</div>
						<div>
							<p class="text-gray-500">Religion</p>
							<p class="font-medium">{selectedStudent.religion}</p>
						</div>
					</div>
				</div>
			</div>

			<div class="mt-6 border-t border-gray-400 pt-4">
				{#if selectedStudent.health_notes}
					<div class="mb-3">
						<p class="text-gray-500">Health Notes</p>
						<p class="font-medium">{selectedStudent.health_notes}</p>
					</div>
				{/if}
				{#if selectedStudent.general_notes}
					<div class="mb-3">
						<p class="text-gray-500">General Notes</p>
						<p class="font-medium">{selectedStudent.general_notes}</p>
					</div>
				{/if}
				<div>
					<p class="text-gray-500">Address</p>
					<p class="font-medium">{selectedStudent.address}</p>
				</div>
			</div>

			<h2 class="text-primary mt-8 mb-4 border-b border-gray-400 pb-2 text-xl font-semibold">
				Guardians
			</h2>
			{#if guardians.length > 0}
				<table class="w-full table-auto border border-gray-300 text-sm">
					<thead>
						<tr class="bg-gray-100">
							<th class="border border-gray-300 p-2">Photo</th>
							<th class="border border-gray-300 p-2 text-left">Name</th>
							<th class="border border-gray-300 p-2 text-left">Relationship</th>
							<th class="border border-gray-300 p-2">Phone</th>
							<th class="border border-gray-300 p-2 text-left">Address</th>
						</tr>
					</thead>
					<tbody>
						{#each guardians as guardian (guardian.id)}
							<tr>
								<td class="border border-gray-300 p-2 text-center">
									<img
										src={guardian.photo}
										alt="guardian photo"
										class="mx-auto h-10 w-10 rounded-full object-cover"
									/>
								</td>
								<td class="border border-gray-300 p-2">{guardian.name}</td>
								<td class="border border-gray-300 p-2">{guardian.relationship}</td>
								<td class="border border-gray-300 p-2 text-center">{guardian.phone}</td>
								<td class="border border-gray-300 p-2">{guardian.address}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			{:else}
				<p class="text-gray-500 italic">No guardians found for this student.</p>
			{/if}
		{:else}
			<p class="text-gray-600 italic">Please select a student from the list.</p>
		{/if}
	</div>
</div>

<style>
	/* Tailwind classes handle styling, no custom CSS needed */
</style>
