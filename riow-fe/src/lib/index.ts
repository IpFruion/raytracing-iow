import { z } from 'zod';

export function hexToRgb(hex: string): Color | null {
	const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
	return result
		? {
				r: parseInt(result[1], 16) / 256.0,
				g: parseInt(result[2], 16) / 256.0,
				b: parseInt(result[3], 16) / 256.0
		  }
		: null;
}

export type Vec3 = { x: number; y: number; z: number };
export type Color = { r: number; g: number; b: number };

export type Material =
	| { materialType: 'lambertain'; color: string }
	| { materialType: 'metal'; color: string; fuzziness: number }
	| { materialType: 'dielectric'; indexOfRefraction: number };

export type Shape = { shapeType: 'sphere'; stationary: true; center: Vec3; radius: number };

export type WorldObjectData = {
	material: Material;
	shape: Shape;
};

export const defaultWorldObjectData: () => WorldObjectData = () => ({
	material: {
		materialType: 'lambertain',
		color: '#f97923'
	},
	shape: {
		shapeType: 'sphere',
		stationary: true,
		center: {
			x: 0,
			y: 1,
			z: 0
		},
		radius: 1
	}
});

export type CameraConfig = {
	samplesPerPixel: number;
	maxDepth: number;
	defocusAngle: number;
	position: Vec3;
	up: Vec3;
	lookAt: Vec3;
};

export type ImageConfig = {
	width: number;
	height: number;
};

const ImageStatusSchema = z.object({
	Rendering: z
		.object({
			cur_pixel: z.number().int(),
			max_pixels: z.number().int()
		})
		.optional(),
	download_url: z.string().url().optional()
});

export type ImageStatus = z.infer<typeof ImageStatusSchema>;

export async function imageGenStatus(url: string): Promise<ImageStatus> {
	const res = await fetch(url);
	return ImageStatusSchema.parse(await res.json());
}

export async function downloadImage(url: string): Promise<string> {
	const res = await fetch(url);
	const imageBlob = await res.blob();
	return URL.createObjectURL(imageBlob);
}
