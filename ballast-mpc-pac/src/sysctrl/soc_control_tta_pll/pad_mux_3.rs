#[doc = "Register `PAD_MUX_3` reader"]
pub struct R(crate::R<PAD_MUX_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_MUX_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_MUX_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_MUX_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_MUX_3` writer"]
pub struct W(crate::W<PAD_MUX_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_MUX_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PAD_MUX_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_MUX_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pad_48` reader - "]
pub struct PAD_48_R(crate::FieldReader<u8, u8>);
impl PAD_48_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_48_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_48_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_48` writer - "]
pub struct PAD_48_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_48_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `pad_49` reader - "]
pub struct PAD_49_R(crate::FieldReader<u8, u8>);
impl PAD_49_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_49_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_49_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_49` writer - "]
pub struct PAD_49_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_49_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `pad_50` reader - "]
pub struct PAD_50_R(crate::FieldReader<u8, u8>);
impl PAD_50_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_50_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_50_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_50` writer - "]
pub struct PAD_50_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_50_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `pad_51` reader - "]
pub struct PAD_51_R(crate::FieldReader<u8, u8>);
impl PAD_51_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_51_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_51_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_51` writer - "]
pub struct PAD_51_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_51_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `pad_52` reader - "]
pub struct PAD_52_R(crate::FieldReader<u8, u8>);
impl PAD_52_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_52_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_52_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_52` writer - "]
pub struct PAD_52_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_52_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `pad_53` reader - "]
pub struct PAD_53_R(crate::FieldReader<u8, u8>);
impl PAD_53_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_53_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_53_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_53` writer - "]
pub struct PAD_53_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_53_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `pad_54` reader - "]
pub struct PAD_54_R(crate::FieldReader<u8, u8>);
impl PAD_54_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_54_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_54_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_54` writer - "]
pub struct PAD_54_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_54_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `pad_55` reader - "]
pub struct PAD_55_R(crate::FieldReader<u8, u8>);
impl PAD_55_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_55_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_55_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_55` writer - "]
pub struct PAD_55_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_55_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `pad_56` reader - "]
pub struct PAD_56_R(crate::FieldReader<u8, u8>);
impl PAD_56_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_56_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_56_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_56` writer - "]
pub struct PAD_56_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_56_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `pad_57` reader - "]
pub struct PAD_57_R(crate::FieldReader<u8, u8>);
impl PAD_57_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_57_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_57_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_57` writer - "]
pub struct PAD_57_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_57_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `pad_58` reader - "]
pub struct PAD_58_R(crate::FieldReader<u8, u8>);
impl PAD_58_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_58_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_58_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_58` writer - "]
pub struct PAD_58_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_58_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `pad_59` reader - "]
pub struct PAD_59_R(crate::FieldReader<u8, u8>);
impl PAD_59_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_59_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_59_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_59` writer - "]
pub struct PAD_59_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_59_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `pad_60` reader - "]
pub struct PAD_60_R(crate::FieldReader<u8, u8>);
impl PAD_60_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_60_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_60_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_60` writer - "]
pub struct PAD_60_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_60_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `pad_61` reader - "]
pub struct PAD_61_R(crate::FieldReader<u8, u8>);
impl PAD_61_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_61_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_61_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_61` writer - "]
pub struct PAD_61_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_61_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `pad_62` reader - "]
pub struct PAD_62_R(crate::FieldReader<u8, u8>);
impl PAD_62_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_62_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_62_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_62` writer - "]
pub struct PAD_62_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_62_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `pad_63` reader - "]
pub struct PAD_63_R(crate::FieldReader<u8, u8>);
impl PAD_63_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_63_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_63_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_63` writer - "]
pub struct PAD_63_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_63_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pad_48(&self) -> PAD_48_R {
        PAD_48_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_49(&self) -> PAD_49_R {
        PAD_49_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pad_50(&self) -> PAD_50_R {
        PAD_50_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pad_51(&self) -> PAD_51_R {
        PAD_51_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pad_52(&self) -> PAD_52_R {
        PAD_52_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_53(&self) -> PAD_53_R {
        PAD_53_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_54(&self) -> PAD_54_R {
        PAD_54_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pad_55(&self) -> PAD_55_R {
        PAD_55_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pad_56(&self) -> PAD_56_R {
        PAD_56_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pad_57(&self) -> PAD_57_R {
        PAD_57_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_58(&self) -> PAD_58_R {
        PAD_58_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_59(&self) -> PAD_59_R {
        PAD_59_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pad_60(&self) -> PAD_60_R {
        PAD_60_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pad_61(&self) -> PAD_61_R {
        PAD_61_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pad_62(&self) -> PAD_62_R {
        PAD_62_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pad_63(&self) -> PAD_63_R {
        PAD_63_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pad_48(&mut self) -> PAD_48_W {
        PAD_48_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_49(&mut self) -> PAD_49_W {
        PAD_49_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pad_50(&mut self) -> PAD_50_W {
        PAD_50_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pad_51(&mut self) -> PAD_51_W {
        PAD_51_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pad_52(&mut self) -> PAD_52_W {
        PAD_52_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_53(&mut self) -> PAD_53_W {
        PAD_53_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_54(&mut self) -> PAD_54_W {
        PAD_54_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pad_55(&mut self) -> PAD_55_W {
        PAD_55_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pad_56(&mut self) -> PAD_56_W {
        PAD_56_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pad_57(&mut self) -> PAD_57_W {
        PAD_57_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_58(&mut self) -> PAD_58_W {
        PAD_58_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_59(&mut self) -> PAD_59_W {
        PAD_59_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pad_60(&mut self) -> PAD_60_W {
        PAD_60_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pad_61(&mut self) -> PAD_61_W {
        PAD_61_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pad_62(&mut self) -> PAD_62_W {
        PAD_62_W { w: self }
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pad_63(&mut self) -> PAD_63_W {
        PAD_63_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The content of these registers can be used to multiplex pads when targeting an ASIC. The forth register (0x1A10_401C) can be used to sets the mux (2 bit select) from pin 48 (bits \\[1:0\\]) to 63 (bits \\[31:30\\]).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_mux_3](index.html) module"]
pub struct PAD_MUX_3_SPEC;
impl crate::RegisterSpec for PAD_MUX_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_mux_3::R](R) reader structure"]
impl crate::Readable for PAD_MUX_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_mux_3::W](W) writer structure"]
impl crate::Writable for PAD_MUX_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_MUX_3 to value 0"]
impl crate::Resettable for PAD_MUX_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
