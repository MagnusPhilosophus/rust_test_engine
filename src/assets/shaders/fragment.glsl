	#version 330 core
	out vec4 color;
	in vec3 pos;

	const vec3 green = vec3(0.48, 0.98 ,0);
	const vec3 brown = vec3(0.51, 0.41 ,0.32);

	void main()
	{
			//color = vec4(abs(tan(pos)), 1.0);
			color = vec4(green.x+(brown.x-green.x)*pos.x/2,
										green.y+(brown.y-green.y)*pos.y/2,
										green.z+(brown.z-green.z)*pos.z/2,
										1.0);
			
	}
