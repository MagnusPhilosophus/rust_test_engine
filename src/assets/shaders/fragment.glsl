	#version 330 core
	out vec4 FragColor;
	in vec3 pos;

	void main()
	{
			FragColor = vec4(1.0, pos.y, 0.0, 1.0);
	}
