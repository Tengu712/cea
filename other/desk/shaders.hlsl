struct VSInput {
    float4 pos : POSITION;
    float4 col : COLOR;
    float2 tex : TEXCOORD;
};

struct PSInput
{
    float4 pos : SV_POSITION;
    float4 col : COLOR;
    float2 tex : TEXCOORD;
};

PSInput vs_main(VSInput input)
{
    PSInput result;

    result.pos = input.pos;
    result.col = input.col;
    result.tex = input.tex;

    return result;
}

float4 ps_main(PSInput input) : SV_TARGET
{
    return input.col;
}
