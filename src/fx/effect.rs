struct Color
{

};

struct Texture
{

};

enum CommonColorOrTextureType
{
    Color(Color),
    Texture(Texture)
};

enum CommonFloatOrParamType
{
    Float(f64),
};

struct Lambert
{
    emission: Option<CommonColorOrTextureType>;
    diffuse: Option<CommonColorOrTextureType>;
    ior: Option<>

};

enum ShaderCommon
{
    Lambert(Lambert),
};

struct Technique<T>
{
    sid: String;
    shader: T;
};

pub type TechniqueCommon = Technique<ShaderCommon>;

struct ProfileCommon
{
    id: Option<String>;
    technique: TechniqueCommon;
};

enum Profile
{
    ProfileCommon(ProfileCommon)
};

struct Effect
{
    id: String;
    name: Option<String>;
    profile: Profile;
}