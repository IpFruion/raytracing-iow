import { hexToRgb, type CameraConfig, type ImageConfig, type WorldObjectData } from '$lib';
import { z } from 'zod';

export type GenerateImageRequest = {
	imageConfig: ImageConfig;
	cameraConfig: CameraConfig;
	world: WorldObjectData[];
};

export const defaultGenerateImageRequest: () => GenerateImageRequest = () => ({
	imageConfig: {
		width: 500,
		height: 300
	},
	cameraConfig: {
		samplesPerPixel: 50,
		maxDepth: 10,
		defocusAngle: 0.6,
		position: { x: 13, y: 2, z: 3 },
		up: { x: 0, y: 1, z: 0 },
		lookAt: { x: 0, y: 0, z: 0 }
	},
	world: [
		{
			material: {
				materialType: 'lambertain',
				color: '#f97923'
			},
			shape: {
				shapeType: 'sphere',
				stationary: true,
				center: {
					x: 0,
					y: -1000,
					z: 0
				},
				radius: 1000
			}
		},
		{
			material: {
				materialType: 'dielectric',
				indexOfRefraction: 1.5
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
		},
		{
			material: {
				materialType: 'lambertain',
				color: '#47059d'
			},
			shape: {
				shapeType: 'sphere',
				stationary: true,
				center: {
					x: -4,
					y: 1,
					z: 0
				},
				radius: 1
			}
		},
		{
			material: {
				materialType: 'metal',
				color: '#ffffff',
				fuzziness: 0.0
			},
			shape: {
				shapeType: 'sphere',
				stationary: true,
				center: {
					x: 4,
					y: 1,
					z: 0
				},
				radius: 1
			}
		}
	]
});

const GenerateImageResponseSchema = z.object({
	id: z.string(),
	status_url: z.string().url()
});

export type GenerateImageResponse = z.infer<typeof GenerateImageResponseSchema>;

export async function generateImage(req: GenerateImageRequest): Promise<GenerateImageResponse> {
	const body = {
		width: req.imageConfig.width,
		height: req.imageConfig.height,
		camera_config: {
			defocus_angle: req.cameraConfig.defocusAngle,
			focus_dist: 10,
			look_at: req.cameraConfig.lookAt,
			max_depth: req.cameraConfig.maxDepth,
			samples_per_pixel: req.cameraConfig.samplesPerPixel,
			up: req.cameraConfig.up,
			pos: req.cameraConfig.position
		},
		objects: req.world.map((o) => {
			let material: any = {};
			switch (o.material.materialType) {
				case 'lambertain': {
					material = { Lambertain: { color: hexToRgb(o.material.color) } };
					break;
				}
				case 'dielectric': {
					material = { Dielectric: { index_of_refraction: o.material.indexOfRefraction } };
					break;
				}
				case 'metal': {
					material = {
						Metal: { color: hexToRgb(o.material.color), fuzziness: o.material.fuzziness }
					};
					break;
				}
			}
			let shape: any = {};
			switch (o.shape.shapeType) {
				case 'sphere': {
					shape = { Sphere: { Stationary: { center: o.shape.center, radius: o.shape.radius } } };
					break;
				}
			}
			return {
				material,
				shape
			};
		})
	};
	console.log(body);

	const res = await fetch('/', {
		headers: {
			'content-type': 'application/json'
		},
		method: 'POST',
		body: JSON.stringify(body)
	});

	return GenerateImageResponseSchema.parse(await res.json());
}
