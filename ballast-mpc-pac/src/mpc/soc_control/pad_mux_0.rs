#[doc = "Register `PAD_MUX_0` reader"]
pub struct R(crate::R<PAD_MUX_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_MUX_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_MUX_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_MUX_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_MUX_0` writer"]
pub struct W(crate::W<PAD_MUX_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_MUX_0_SPEC>;
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
impl From<crate::W<PAD_MUX_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_MUX_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pad_0` reader - "]
pub type PAD_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_0` writer - "]
pub type PAD_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_1` reader - "]
pub type PAD_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_1` writer - "]
pub type PAD_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_2` reader - "]
pub type PAD_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_2` writer - "]
pub type PAD_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_3` reader - "]
pub type PAD_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_3` writer - "]
pub type PAD_3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_4` reader - "]
pub type PAD_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_4` writer - "]
pub type PAD_4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_5` reader - "]
pub type PAD_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_5` writer - "]
pub type PAD_5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_6` reader - "]
pub type PAD_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_6` writer - "]
pub type PAD_6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_7` reader - "]
pub type PAD_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_7` writer - "]
pub type PAD_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_8` reader - "]
pub type PAD_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_8` writer - "]
pub type PAD_8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_9` reader - "]
pub type PAD_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_9` writer - "]
pub type PAD_9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_10` reader - "]
pub type PAD_10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_10` writer - "]
pub type PAD_10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_11` reader - "]
pub type PAD_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_11` writer - "]
pub type PAD_11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_12` reader - "]
pub type PAD_12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_12` writer - "]
pub type PAD_12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_13` reader - "]
pub type PAD_13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_13` writer - "]
pub type PAD_13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_14` reader - "]
pub type PAD_14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_14` writer - "]
pub type PAD_14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `pad_15` reader - "]
pub type PAD_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_15` writer - "]
pub type PAD_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAD_MUX_0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pad_0(&self) -> PAD_0_R {
        PAD_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_1(&self) -> PAD_1_R {
        PAD_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pad_2(&self) -> PAD_2_R {
        PAD_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pad_3(&self) -> PAD_3_R {
        PAD_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pad_4(&self) -> PAD_4_R {
        PAD_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_5(&self) -> PAD_5_R {
        PAD_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_6(&self) -> PAD_6_R {
        PAD_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pad_7(&self) -> PAD_7_R {
        PAD_7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pad_8(&self) -> PAD_8_R {
        PAD_8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pad_9(&self) -> PAD_9_R {
        PAD_9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_10(&self) -> PAD_10_R {
        PAD_10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_11(&self) -> PAD_11_R {
        PAD_11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pad_12(&self) -> PAD_12_R {
        PAD_12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pad_13(&self) -> PAD_13_R {
        PAD_13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pad_14(&self) -> PAD_14_R {
        PAD_14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pad_15(&self) -> PAD_15_R {
        PAD_15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pad_0(&mut self) -> PAD_0_W<0> {
        PAD_0_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn pad_1(&mut self) -> PAD_1_W<2> {
        PAD_1_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pad_2(&mut self) -> PAD_2_W<4> {
        PAD_2_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn pad_3(&mut self) -> PAD_3_W<6> {
        PAD_3_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn pad_4(&mut self) -> PAD_4_W<8> {
        PAD_4_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn pad_5(&mut self) -> PAD_5_W<10> {
        PAD_5_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn pad_6(&mut self) -> PAD_6_W<12> {
        PAD_6_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn pad_7(&mut self) -> PAD_7_W<14> {
        PAD_7_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn pad_8(&mut self) -> PAD_8_W<16> {
        PAD_8_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn pad_9(&mut self) -> PAD_9_W<18> {
        PAD_9_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn pad_10(&mut self) -> PAD_10_W<20> {
        PAD_10_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn pad_11(&mut self) -> PAD_11_W<22> {
        PAD_11_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn pad_12(&mut self) -> PAD_12_W<24> {
        PAD_12_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn pad_13(&mut self) -> PAD_13_W<26> {
        PAD_13_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn pad_14(&mut self) -> PAD_14_W<28> {
        PAD_14_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn pad_15(&mut self) -> PAD_15_W<30> {
        PAD_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The content of these registers can be used to multiplex pads when targeting an ASIC. The first register (0x1A10_4010) can be used to set the mux (2 bit select) from pin 0 (bits \\[1:0\\]) to 15 (bits \\[31:30\\]).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_mux_0](index.html) module"]
pub struct PAD_MUX_0_SPEC;
impl crate::RegisterSpec for PAD_MUX_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_mux_0::R](R) reader structure"]
impl crate::Readable for PAD_MUX_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_mux_0::W](W) writer structure"]
impl crate::Writable for PAD_MUX_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_MUX_0 to value 0"]
impl crate::Resettable for PAD_MUX_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
