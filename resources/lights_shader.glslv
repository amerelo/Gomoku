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
	vec2 coord = gl_FragCoord.xy / u_ScreenSize;
	vec2 rel = coord - u_Pos;
	float theta = atan(rel.y, rel.x);
	float ox = degrees(theta) / 360.0;
	if (ox < 0)
	{
		ox += 1.0;
	}
	float r = length(rel);
	float occl = texture(t_Texture, vec2(ox, 0.5)).r * 2.0;

	float intensity = 0.0;
	if (r < occl)
	{
		vec2 g = u_ScreenSize / u_ScreenSize.y;
		float p = u_Strength + u_Glow;
		float d = distance(g * coord, g * u_Pos);
		intensity = clamp(p/(d*d), 0.0, 0.6);
	}

	Target0 = mix(vec4(0.0, 0.0, 0.0, 1.0), vec4(u_LightColor.rgb, 1.0), intensity);
}