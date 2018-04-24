#version 150 core

uniform sampler2D t_Texture;
in vec2 v_Uv;
out vec4 Target0;

layout (std140) uniform Light {
	vec4 u_LightColor;
	vec4 u_ShadowColor;
	vec2 u_Pos;
	vec2 u_ScreenSize;
	float u_Glow;
	float u_Strength;
};

void main() {
	float dist = 1.0;
	float theta = radians(v_Uv.x * 360.0);
	vec2 dir = vec2(cos(theta), sin(theta));
	for(int i = 0; i < 1024; i++)
	{
		float fi = i;
		float r = fi / 1024.0;
		vec2 rel = r * dir;
		vec2 p = clamp(u_Pos+rel, 0.0, 1.0);
		if (texture(t_Texture, p).a > 0.8)
		{
			dist = distance(u_Pos, p) * 0.5;
			break;
		}
	}

	float others = dist == 1.0 ? 0.0 : dist;
	Target0 = vec4(dist, others, others, 1.0);
}