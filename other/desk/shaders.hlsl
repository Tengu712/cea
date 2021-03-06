// Parameters
//   x: 0.0 -> no image texture
//   y: 1.0 -> overlay
//   z:
//   w:

Texture2D diffuseTexture;
SamplerState diffuseTextureSampler;

cbuffer mats : register(b0) {
    float4x4 mat_scl;
    float4x4 mat_rtx;
    float4x4 mat_rty;
    float4x4 mat_rtz;
    float4x4 mat_trs;
    float4x4 mat_view;
    float4x4 mat_proj;
    float4 vec_col;
    float4 vec_prm;
}

struct VSInput {
    float4 pos : POSITION;
    float4 col : COLOR;
    float2 tex : TEXCOORD;
};

struct PSInput
{
    float4 pos : SV_POSITION;
    float4 col : COLOR0;
    float2 tex : TEXCOORD;
    float4 prm : COLOR1;
};

PSInput vs_main(VSInput input)
{
    PSInput result;

    result.col = input.col * vec_col;
    result.pos = mul(input.pos, mat_scl);
    result.pos = mul(result.pos, mat_rtx);
    result.pos = mul(result.pos, mat_rty);
    result.pos = mul(result.pos, mat_rtz);
    result.pos = mul(result.pos, mat_trs);
    result.pos = mul(result.pos, mat_view);
    result.pos = mul(result.pos, mat_proj);
    result.tex = input.tex;
    result.prm = vec_prm;

    return result;
}

float4 ps_main(PSInput input) : SV_TARGET
{
    if (input.prm.x != 0.0) {
        float4 col = diffuseTexture.Sample(diffuseTextureSampler, input.tex);
        if (input.prm.y == 1.0) {
            if (col.x + col.y + col.z + col.w < 2.0)
                return col * input.col * 2.0;
            else
                return 2.0 * (col + input.col - col * input.col) - 1.0;
        }
        else
            return input.col * col;
    } else
        return input.col;
}
