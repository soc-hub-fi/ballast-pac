#[doc = "Register `REG_SS_CLK_CTRL2` reader"]
pub struct R(crate::R<REG_SS_CLK_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_SS_CLK_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_SS_CLK_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_SS_CLK_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_SS_CLK_CTRL2` writer"]
pub struct W(crate::W<REG_SS_CLK_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_SS_CLK_CTRL2_SPEC>;
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
impl From<crate::W<REG_SS_CLK_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_SS_CLK_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPC_CLK_CTRL` reader - "]
pub struct MPC_CLK_CTRL_R(crate::FieldReader<u8, u8>);
impl MPC_CLK_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MPC_CLK_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPC_CLK_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPC_CLK_CTRL` writer - "]
pub struct MPC_CLK_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> MPC_CLK_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ICN_CLK_CTRL` reader - "]
pub struct ICN_CLK_CTRL_R(crate::FieldReader<u8, u8>);
impl ICN_CLK_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ICN_CLK_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICN_CLK_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICN_CLK_CTRL` writer - "]
pub struct ICN_CLK_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ICN_CLK_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `C2C_CLK_CTRL` reader - "]
pub struct C2C_CLK_CTRL_R(crate::FieldReader<u8, u8>);
impl C2C_CLK_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        C2C_CLK_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2C_CLK_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2C_CLK_CTRL` writer - "]
pub struct C2C_CLK_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> C2C_CLK_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mpc_clk_ctrl(&self) -> MPC_CLK_CTRL_R {
        MPC_CLK_CTRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn icn_clk_ctrl(&self) -> ICN_CLK_CTRL_R {
        ICN_CLK_CTRL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn c2c_clk_ctrl(&self) -> C2C_CLK_CTRL_R {
        C2C_CLK_CTRL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mpc_clk_ctrl(&mut self) -> MPC_CLK_CTRL_W {
        MPC_CLK_CTRL_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn icn_clk_ctrl(&mut self) -> ICN_CLK_CTRL_W {
        ICN_CLK_CTRL_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn c2c_clk_ctrl(&mut self) -> C2C_CLK_CTRL_W {
        C2C_CLK_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem Clock selection. Bit definition *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_ss_clk_ctrl2](index.html) module"]
pub struct REG_SS_CLK_CTRL2_SPEC;
impl crate::RegisterSpec for REG_SS_CLK_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_ss_clk_ctrl2::R](R) reader structure"]
impl crate::Readable for REG_SS_CLK_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_ss_clk_ctrl2::W](W) writer structure"]
impl crate::Writable for REG_SS_CLK_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_SS_CLK_CTRL2 to value 0"]
impl crate::Resettable for REG_SS_CLK_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
